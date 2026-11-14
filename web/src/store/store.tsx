import {
  createContext,
  type Dispatch,
  useContext,
  useEffect,
  useReducer,
  useRef,
  type ReactNode,
} from "react";
import { api, type RegenRequest, type RegisterRequest } from "../api/client";
import { buildVoteReceipt, pathSeg } from "../lib/civic";

// ── Types ─────────────────────────────────────────────────────────
export type View = "feed" | "governance" | "identity";

export interface Identity {
  nullifier: string; // current rolling seed — bound to posts & votes
  handle: string; // chosen username
  verified: boolean;
  username: string;
  uid: string; // current Citizen-UID seed (rotates each login)
  password: string; // 16-char login password
  expiresAt: number; // ms — password/session expiry
  loginCount: number;
  phoneMasked?: string;
  country?: string;
}

export interface SubTopic {
  id: string; // e.g. st/transport-metro (sent as subtopic_id)
  slug: string; // e.g. metro
}

export interface Topic {
  id: string; // t/<ULID> from POST /citizen/t
  slug: string;
  createdAt: number;
  subtopics: SubTopic[];
  seeded?: boolean;
}

export interface Poll {
  voteId?: string;
  options: string[];
  closesAtBlock: number;
  tally: Record<string, number>;
  myVote?: string;
  lastReceipt?: { nullifier_hex: string; seq: number };
  revealed: boolean;
}

export interface Post {
  id: string; // o/<ULID>
  topicId: string;
  topicSlug: string;
  subtopicId: string;
  subtopicSlug: string;
  authorHandle: string;
  authorNullifier: string;
  content: string;
  cid?: string;
  contentHash?: string;
  verdict?: string;
  createdAt: number;
  poll?: Poll;
}

export interface Toast {
  id: number;
  kind: "ok" | "err" | "info";
  text: string;
}

interface State {
  identity: Identity | null;
  topics: Topic[];
  posts: Post[];
  view: View;
  topicFilter: string | null; // topicId (community)
  subFilter: string | null; // subtopicId
  toasts: Toast[];
}

type Action =
  | { type: "SET_IDENTITY"; identity: Identity }
  | { type: "CLEAR_IDENTITY" }
  | { type: "ADD_TOPIC"; topic: Topic }
  | { type: "ADD_SUBTOPIC"; topicId: string; sub: SubTopic }
  | { type: "ADD_POST"; post: Post }
  | {
      type: "CAST";
      postId: string;
      option: string;
      voteId: string;
      receipt: { nullifier_hex: string; seq: number };
    }
  | { type: "REVEAL"; postId: string }
  | { type: "SET_VIEW"; view: View }
  | { type: "SET_TOPIC"; topicId: string | null }
  | { type: "SET_SUB"; subId: string | null }
  | { type: "TOAST"; toast: Toast }
  | { type: "DISMISS"; id: number };

// ── Starter communities (structure only — NO posts, NO votes) ──────
function sub(topicSlug: string, s: string): SubTopic {
  return { id: `st/${topicSlug}-${s}`, slug: s };
}
function seedTopics(): Topic[] {
  const now = Date.now();
  const mk = (slug: string, subs: string[]): Topic => ({
    id: `t/seed-${slug}`,
    slug,
    createdAt: now,
    seeded: true,
    subtopics: subs.map((s) => sub(slug, s)),
  });
  return [
    mk("infrastructure", ["general", "roads", "water", "power"]),
    mk("transport", ["general", "metro", "buses"]),
    mk("environment", ["general", "air-quality", "green-spaces"]),
  ];
}

const STORAGE_KEY = "janmat.v2";

function initialState(): State {
  let persisted: Partial<State> | null = null;
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) persisted = JSON.parse(raw);
  } catch {
    persisted = null;
  }
  return {
    identity: persisted?.identity ?? null,
    topics: persisted?.topics?.length ? persisted.topics : seedTopics(),
    posts: persisted?.posts ?? [],
    view: "feed",
    topicFilter: null,
    subFilter: null,
    toasts: [],
  };
}

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case "SET_IDENTITY":
      return { ...state, identity: action.identity };
    case "CLEAR_IDENTITY":
      return { ...state, identity: null };
    case "ADD_TOPIC":
      if (state.topics.some((t) => t.id === action.topic.id)) return state;
      return { ...state, topics: [action.topic, ...state.topics] };
    case "ADD_SUBTOPIC":
      return {
        ...state,
        topics: state.topics.map((t) =>
          t.id === action.topicId &&
          !t.subtopics.some((s) => s.id === action.sub.id)
            ? { ...t, subtopics: [...t.subtopics, action.sub] }
            : t,
        ),
      };
    case "ADD_POST":
      return { ...state, posts: [action.post, ...state.posts] };
    case "CAST":
      return {
        ...state,
        posts: state.posts.map((post) => {
          if (post.id !== action.postId || !post.poll) return post;
          const tally = { ...post.poll.tally };
          tally[action.option] = (tally[action.option] ?? 0) + 1;
          return {
            ...post,
            poll: {
              ...post.poll,
              voteId: action.voteId,
              myVote: action.option,
              tally,
              lastReceipt: action.receipt,
            },
          };
        }),
      };
    case "REVEAL":
      return {
        ...state,
        posts: state.posts.map((post) =>
          post.id === action.postId && post.poll
            ? { ...post, poll: { ...post.poll, revealed: true } }
            : post,
        ),
      };
    case "SET_VIEW":
      return { ...state, view: action.view };
    case "SET_TOPIC":
      return { ...state, view: "feed", topicFilter: action.topicId, subFilter: null };
    case "SET_SUB":
      return { ...state, subFilter: action.subId };
    case "TOAST":
      return { ...state, toasts: [...state.toasts, action.toast] };
    case "DISMISS":
      return { ...state, toasts: state.toasts.filter((t) => t.id !== action.id) };
    default:
      return state;
  }
}

// ── Context ───────────────────────────────────────────────────────
const Ctx = createContext<{ state: State; dispatch: Dispatch<Action> } | null>(
  null,
);

export function StoreProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, undefined, initialState);
  useEffect(() => {
    const { identity, topics, posts } = state;
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({ identity, topics, posts }));
    } catch {
      /* ignore quota */
    }
  }, [state.identity, state.topics, state.posts]);
  return <Ctx.Provider value={{ state, dispatch }}>{children}</Ctx.Provider>;
}

// ── Operations (each hits the real API) ───────────────────────────
export function useCivic() {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("useCivic must be used within StoreProvider");
  const { state, dispatch } = ctx;
  const ref = useRef(state);
  ref.current = state;

  let seq = 0;
  const toast = (kind: Toast["kind"], text: string) =>
    dispatch({ type: "TOAST", toast: { id: Date.now() + seq++, kind, text } });

  const setView = (view: View) => dispatch({ type: "SET_VIEW", view });
  const setTopic = (topicId: string | null) => dispatch({ type: "SET_TOPIC", topicId });
  const setSub = (subId: string | null) => dispatch({ type: "SET_SUB", subId });
  const dismissToast = (id: number) => dispatch({ type: "DISMISS", id });

  function requireIdentity(): Identity | null {
    const id = ref.current.identity;
    if (!id) {
      toast("info", "Register or log in first.");
      setView("identity");
    }
    return id;
  }

  const logout = () => {
    dispatch({ type: "CLEAR_IDENTITY" });
    toast("info", "Signed out.");
  };

  const expireSession = () => {
    dispatch({ type: "CLEAR_IDENTITY" });
    toast("info", "Your 24-hour password expired. Generate a new one to continue.");
    setView("identity");
  };

  type AuthResult = { ok: boolean; error?: string; field?: string; password?: string };
  const errInfo = (e: unknown) => {
    const a = e as { message?: string; body?: { error?: string; field?: string } };
    return { msg: a.body?.error ?? a.message ?? "Request failed", field: a.body?.field };
  };

  const register = async (fields: RegisterRequest): Promise<AuthResult> => {
    try {
      const r = await api.register(fields);
      dispatch({
        type: "SET_IDENTITY",
        identity: {
          nullifier: r.uid,
          handle: r.username,
          username: r.username,
          uid: r.uid,
          password: r.password,
          expiresAt: r.password_expires_at_ms,
          loginCount: r.login_count,
          verified: true,
          phoneMasked: r.masked_phone,
          country: r.citizenship_country,
        },
      });
      toast("ok", `Registered — save your password: ${r.password}`);
      // Stay on the identity view so the citizen can copy their password.
      return { ok: true, password: r.password };
    } catch (e) {
      const { msg, field } = errInfo(e);
      toast("err", msg);
      return { ok: false, error: msg, field };
    }
  };

  const login = async (password: string): Promise<AuthResult> => {
    try {
      const r = await api.login(password.trim());
      dispatch({
        type: "SET_IDENTITY",
        identity: {
          nullifier: r.uid,
          handle: r.username,
          username: r.username,
          uid: r.uid,
          password: password.trim(),
          expiresAt: r.expires_at_ms,
          loginCount: r.login_count,
          verified: true,
          country: r.citizenship_country,
        },
      });
      toast("ok", `Welcome back, @${r.username}.`);
      setView("feed");
      return { ok: true };
    } catch (e) {
      const { msg, field } = errInfo(e);
      toast("err", msg);
      return { ok: false, error: msg, field };
    }
  };

  const regeneratePassword = async (fields: RegenRequest): Promise<AuthResult> => {
    try {
      const r = await api.regeneratePassword(fields);
      toast("ok", `New password: ${r.password}`);
      const li = await login(r.password); // log straight in with the fresh password
      return { ok: li.ok, password: r.password, error: li.error };
    } catch (e) {
      const { msg, field } = errInfo(e);
      toast("err", msg);
      return { ok: false, error: msg, field };
    }
  };

  const createTopic = async (rawSlug: string): Promise<Topic | null> => {
    const slug = rawSlug.trim().toLowerCase().replace(/[^a-z0-9-]+/g, "-").replace(/^-|-$/g, "");
    if (slug.length < 2) {
      toast("err", "Community name must be at least 2 characters.");
      return null;
    }
    try {
      const r = await api.createTopic({ slug });
      const topic: Topic = {
        id: r.topic_id,
        slug,
        createdAt: Date.now(),
        subtopics: [sub(slug, "general")],
      };
      dispatch({ type: "ADD_TOPIC", topic });
      dispatch({ type: "SET_TOPIC", topicId: topic.id });
      toast("ok", `Community j/${slug} created.`);
      return topic;
    } catch (e) {
      toast("err", `Could not create community: ${(e as Error).message}`);
      return null;
    }
  };

  const addSubtopic = (topicId: string, rawSlug: string): SubTopic | null => {
    const topic = ref.current.topics.find((t) => t.id === topicId);
    if (!topic) return null;
    const slug = rawSlug.trim().toLowerCase().replace(/[^a-z0-9-]+/g, "-").replace(/^-|-$/g, "");
    if (slug.length < 2) {
      toast("err", "Subtopic name must be at least 2 characters.");
      return null;
    }
    if (topic.subtopics.some((s) => s.slug === slug)) {
      toast("info", `Subtopic st/${slug} already exists.`);
      return topic.subtopics.find((s) => s.slug === slug)!;
    }
    const s = sub(topic.slug, slug);
    dispatch({ type: "ADD_SUBTOPIC", topicId, sub: s });
    toast("ok", `Subtopic st/${slug} added.`);
    return s;
  };

  const publishPost = async (opts: {
    topicId: string;
    subtopicId: string;
    content: string;
    pollOptions?: string[];
  }): Promise<boolean> => {
    const id = requireIdentity();
    if (!id) return false;
    const topic = ref.current.topics.find((t) => t.id === opts.topicId);
    const st = topic?.subtopics.find((s) => s.id === opts.subtopicId);
    if (!topic || !st) {
      toast("err", "Choose a community and subtopic first.");
      return false;
    }
    try {
      let outcome = await api.postOpinion({
        subtopic_id: st.id,
        content: opts.content,
        creator_nullifier: id.nullifier,
        clone_acknowledged: false,
      });
      if (outcome.kind === "clone") {
        toast("info", "A similar opinion exists — posting anyway.");
        outcome = await api.postOpinion({
          subtopic_id: st.id,
          content: opts.content,
          creator_nullifier: id.nullifier,
          clone_acknowledged: true,
        });
      }
      if (outcome.kind === "blocked") {
        toast("err", "Blocked by the content-safety policy.");
        return false;
      }
      if (outcome.kind === "review") {
        toast("info", "Held for moderator review before it goes live.");
        return false;
      }
      if (outcome.kind !== "ok") return false;

      const d = outcome.data;
      let poll: Poll | undefined;
      const options = (opts.pollOptions ?? []).map((s) => s.trim()).filter(Boolean);
      if (options.length >= 2) {
        const v = await api.createVote({
          opinion_id: d.opinion_id,
          options,
          closes_at_block: 99999,
        });
        poll = {
          voteId: v.vote_id,
          options: v.options,
          closesAtBlock: v.closes_at_block,
          tally: Object.fromEntries(v.options.map((o) => [o, 0])),
          revealed: false,
        };
      }

      dispatch({
        type: "ADD_POST",
        post: {
          id: d.opinion_id,
          topicId: topic.id,
          topicSlug: topic.slug,
          subtopicId: st.id,
          subtopicSlug: st.slug,
          authorHandle: id.handle,
          authorNullifier: id.nullifier,
          content: opts.content,
          cid: d.content_cid,
          contentHash: d.content_hash,
          verdict: d.verdict,
          createdAt: Date.now(),
          poll,
        },
      });
      toast("ok", "Opinion posted.");
      return true;
    } catch (e) {
      toast("err", `Post failed: ${(e as Error).message}`);
      return false;
    }
  };

  const vote = async (postId: string, option: string) => {
    const id = requireIdentity();
    if (!id) return;
    const post = ref.current.posts.find((x) => x.id === postId);
    if (!post?.poll) return;
    if (post.poll.myVote) {
      toast("info", "You already voted in this poll.");
      return;
    }
    try {
      let voteId = post.poll.voteId;
      if (!voteId) {
        const v = await api.createVote({
          opinion_id: post.id,
          options: post.poll.options,
          closes_at_block: post.poll.closesAtBlock,
        });
        voteId = v.vote_id;
      }
      const key = pathSeg(voteId);
      const receipt = buildVoteReceipt(key, id.nullifier, option);
      const cast = await api.castVote(key, { zk_receipt: receipt });
      dispatch({
        type: "CAST",
        postId,
        option,
        voteId,
        receipt: { nullifier_hex: cast.nullifier_hex, seq: cast.seq },
      });
      toast("ok", "Vote cast and verified.");
    } catch (e) {
      toast("err", `Vote failed: ${(e as Error).message}`);
    }
  };

  const reveal = async (postId: string) => {
    const post = ref.current.posts.find((x) => x.id === postId);
    if (!post?.poll?.voteId) {
      dispatch({ type: "REVEAL", postId });
      return;
    }
    try {
      await api.revealVote(post.poll.voteId);
      dispatch({ type: "REVEAL", postId });
      toast("ok", "Results revealed.");
    } catch (e) {
      toast("err", `Reveal failed: ${(e as Error).message}`);
    }
  };

  return {
    state,
    setView,
    setTopic,
    setSub,
    toast,
    dismissToast,
    register,
    login,
    regeneratePassword,
    logout,
    expireSession,
    createTopic,
    addSubtopic,
    publishPost,
    vote,
    reveal,
  };
}
