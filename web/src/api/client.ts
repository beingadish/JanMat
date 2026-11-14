import type {
  AddCommentResponse,
  BallotItem,
  CastBallotResponse,
  CastVoteRequest,
  CastVoteResponse,
  ConstitutionResponse,
  CreateProposalRequest,
  CreateProposalResponse,
  CreateTopicRequest,
  CreateTopicResponse,
  CreateVoteRequest,
  CreateVoteResponse,
  DraftMotionResponse,
  HealthResponse,
  KycHealthResponse,
  KycVerifyResponse,
  PulseResponse,
  RecallRequest,
  RecallResponse,
  ReshareCommentResponse,
  SubmitOpinionRequest,
  SubmitOpinionResponse,
  VerifyReceiptResponse,
  VoteResultsResponse,
} from "./types";

// All requests are prefixed with this base. Defaults to "/api", which is
// proxied to janmat-api by the Vite dev server and by nginx in production.
export const API_BASE: string =
  (import.meta.env.VITE_API_BASE_URL as string | undefined) ?? "/api";

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
    public body?: unknown,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function parseBody(res: Response): Promise<unknown> {
  const text = await res.text();
  if (!text) return null;
  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<T> {
  const headers: Record<string, string> = {};
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
  }

  // Attach citizen authentication token if available in local session
  if (typeof localStorage !== "undefined") {
    try {
      const raw = localStorage.getItem("janmat.v2");
      if (raw) {
        const parsed = JSON.parse(raw);
        if (parsed?.identity?.uid) {
          headers["Authorization"] = `Bearer ${parsed.identity.uid}`;
        }
      }
    } catch {}
  }

  const init: RequestInit = {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  };

  let res: Response;
  try {
    res = await fetch(`${API_BASE}${path}`, init);
  } catch (e) {
    throw new ApiError(0, `Network error: ${(e as Error).message}`);
  }

  const data = await parseBody(res);
  if (!res.ok) {
    const msg =
      (data && typeof data === "object" && "error" in data
        ? String((data as { error: unknown }).error)
        : undefined) ?? `${res.status} ${res.statusText}`;
    throw new ApiError(res.status, msg, data);
  }
  return data as T;
}

// Namespace ids ("v/01H…") carry a slash; API path params take only the ULID.
const seg = (id: string) => (id.includes("/") ? id.slice(id.indexOf("/") + 1) : id);

export interface RegisterRequest {
  username: string;
  pan: string;
  aadhaar: string;
  dob: string;
  phone: string;
}
export interface RegisterResponse {
  success: boolean;
  username: string;
  uid: string;
  nullifier_seed_commitment: string;
  password: string;
  password_expires_at_ms: number;
  login_count: number;
  citizenship_country: string;
  masked_aadhaar: string;
  masked_pan: string;
  masked_phone: string;
  phone_verified: boolean;
}
export interface LoginResponse {
  success: boolean;
  username: string;
  uid: string;
  prev_uid: string;
  login_count: number;
  expires_at_ms: number;
  citizenship_country: string;
}
export interface RegenRequest {
  username: string;
  pan: string;
  aadhaar: string;
  dob: string;
}
export interface RegenResponse {
  success: boolean;
  password: string;
  password_expires_at_ms: number;
}
export interface UsernameAvailability {
  username: string;
  available: boolean;
  reason: string | null;
}

// The content pipeline signals its verdict via HTTP status:
//   200 accepted · 202 held for review · 409 clone warning · 422 blocked.
export type OpinionOutcome =
  | { kind: "ok"; data: SubmitOpinionResponse }
  | { kind: "review" }
  | { kind: "clone" }
  | { kind: "blocked" };

export const api = {
  // ── Public reads ────────────────────────────────────────────────
  health: () => request<HealthResponse>("GET", "/health"),
  kycHealth: () => request<KycHealthResponse>("GET", "/citizen/kyc/health"),
  constitution: () => request<ConstitutionResponse>("GET", "/pub/constitution"),
  listProposals: () => request<unknown[]>("GET", "/pub/proposals"),
  listTopics: () => request<unknown[]>("GET", "/pub/t"),
  getTopic: (id: string) =>
    request<unknown>("GET", `/pub/t/${encodeURIComponent(seg(id))}`),
  voteResults: (topicId: string, voteId: string) =>
    request<VoteResultsResponse>(
      "GET",
      `/pub/t/${encodeURIComponent(seg(topicId))}/v/${encodeURIComponent(seg(voteId))}/results`,
    ),
  usernameAvailable: (name: string) =>
    request<UsernameAvailability>(
      "GET",
      `/citizen/username/${encodeURIComponent(name)}`,
    ),

  // ── Pulse & Ballots (Civic Pulse Redesign) ──────────────────────
  getPulse: () => request<PulseResponse>("GET", "/pub/pulse"),
  listBallots: (category?: string, status?: string) => {
    const params = new URLSearchParams();
    if (category && category !== "all") params.append("category", category);
    if (status && status !== "all") params.append("status", status);
    const qs = params.toString();
    return request<BallotItem[]>("GET", `/pub/ballots${qs ? `?${qs}` : ""}`);
  },
  getBallot: (id: string) =>
    request<BallotItem>("GET", `/pub/ballots/${encodeURIComponent(id)}`),
  castBallotVote: (ballotId: string, choice: string, voterToken?: string) => {
    let token = voterToken;
    if (!token && typeof localStorage !== "undefined") {
      try {
        const raw = localStorage.getItem("janmat.v2");
        if (raw) {
          const parsed = JSON.parse(raw);
          token = parsed?.identity?.uid;
        }
      } catch {}
    }

    if (!token) {
      throw new ApiError(
        401,
        "Authentication required: You must be logged in as a verified citizen to cast a ballot."
      );
    }

    return request<CastBallotResponse>("POST", "/citizen/ballots/vote", {
      ballot_id: ballotId,
      choice,
      voter_token: token,
    });
  },
  draftMotion: (title: string, summary: string, circleId: string, authorName?: string) =>
    request<DraftMotionResponse>("POST", "/citizen/ballots/draft", {
      title,
      summary,
      circle_id: circleId,
      author_name: authorName,
    }),
  addComment: (ballotId: string, content: string, authorName?: string, authorHandle?: string) =>
    request<AddCommentResponse>(
      "POST",
      `/citizen/ballots/${encodeURIComponent(ballotId)}/comments`,
      {
        content,
        author_name: authorName,
        author_handle: authorHandle,
      },
    ),
  reshareComment: (commentId: string, question: string, authorName?: string, circleId?: string) =>
    request<ReshareCommentResponse>("POST", "/citizen/ballots/reshare", {
      comment_id: commentId,
      question,
      author_name: authorName,
      circle_id: circleId,
    }),
  upvoteComment: (commentId: string) =>
    request<{ accepted: boolean; comment_id: string; upvotes: number }>(
      "POST",
      `/citizen/comments/${encodeURIComponent(commentId)}/upvote`,
    ),
  verifyReceipt: (receiptHash: string) =>
    request<VerifyReceiptResponse>("POST", "/pub/verify-receipt", {
      receipt_hash: receiptHash,
    }),

  // ── Citizen writes ──────────────────────────────────────────────
  createTopic: (b: CreateTopicRequest) =>
    request<CreateTopicResponse>("POST", "/citizen/t", b),
  submitOpinion: (b: SubmitOpinionRequest) =>
    request<SubmitOpinionResponse>("POST", "/citizen/o", b),
  // Like submitOpinion, but maps the pipeline's status codes to an outcome
  // instead of throwing on 202/409/422.
  postOpinion: async (b: SubmitOpinionRequest): Promise<OpinionOutcome> => {
    let res: Response;
    try {
      res = await fetch(`${API_BASE}/citizen/o`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(b),
      });
    } catch (e) {
      throw new ApiError(0, `Network error: ${(e as Error).message}`);
    }
    if (res.status === 202) return { kind: "review" };
    if (res.status === 409) return { kind: "clone" };
    if (res.status === 422) return { kind: "blocked" };
    const data = await parseBody(res);
    if (!res.ok)
      throw new ApiError(res.status, `${res.status} ${res.statusText}`, data);
    return { kind: "ok", data: data as SubmitOpinionResponse };
  },
  createVote: (b: CreateVoteRequest) =>
    request<CreateVoteResponse>("POST", "/citizen/v", b),
  castVote: (voteId: string, b: CastVoteRequest) =>
    request<CastVoteResponse>(
      "POST",
      `/citizen/v/${encodeURIComponent(seg(voteId))}/cast`,
      b,
    ),
  revealVote: (voteId: string) =>
    request<null>("POST", `/citizen/v/${encodeURIComponent(seg(voteId))}/reveal`),
  register: (b: RegisterRequest) =>
    request<RegisterResponse>("POST", "/citizen/register", b),
  login: (password: string) =>
    request<LoginResponse>("POST", "/citizen/login", { password }),
  regeneratePassword: (b: RegenRequest) =>
    request<RegenResponse>("POST", "/citizen/password", b),
  createProposal: (b: CreateProposalRequest) =>
    request<CreateProposalResponse>("POST", "/citizen/proposals", b),
  signRecall: (topicId: string, b: RecallRequest) =>
    request<RecallResponse>(
      "POST",
      `/citizen/recall/${encodeURIComponent(seg(topicId))}`,
      b,
    ),

  // ── KYC (multipart) ─────────────────────────────────────────────
  uploadKyc: async (file: File, shareCode: string): Promise<KycVerifyResponse> => {
    const form = new FormData();
    form.append("file", file);
    form.append("share_code", shareCode);
    let res: Response;
    try {
      res = await fetch(`${API_BASE}/citizen/kyc/upload`, {
        method: "POST",
        body: form,
      });
    } catch (e) {
      throw new ApiError(0, `Network error: ${(e as Error).message}`);
    }
    const data = await parseBody(res);
    if (!res.ok) {
      const msg =
        data && typeof data === "object" && "error" in data
          ? String((data as { error: unknown }).error)
          : `${res.status} ${res.statusText}`;
      throw new ApiError(res.status, msg, data);
    }
    return data as KycVerifyResponse;
  },
};
