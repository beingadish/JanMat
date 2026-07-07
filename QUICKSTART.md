# JanMat — Quickstart (Web UI · API · DB · Postman)

This adds a **full civic-polls product** on top of the existing `janmat-api`:
a Vite + React web app where verified citizens register, post opinions, open
**Twitter-style polls**, vote, and watch results — every action calls the real
API. It ships with a one-command Docker stack and a Postman collection.

```
┌────────────┐    /api/*   ┌─────────────┐   DATABASE_URL   ┌────────────┐
│  web       │ ─────────▶  │  api        │ ───────────────▶ │  db        │
│ nginx +    │  (reverse   │ janmat-api  │                  │ postgres16 │
│ React SPA  │   proxy)    │ (Axum)      │                  │            │
└────────────┘             └─────────────┘                  └────────────┘
   :8080                       :3000                            :5432
```

The browser only ever talks to the web origin; nginx reverse-proxies `/api/*`
to the backend, so there is **no CORS** to configure.

---

## 1. Run everything (one command)

```bash
docker compose up --build
```

Then open **http://localhost:8080**.

| Service | URL / Port | Notes |
|---------|-----------|-------|
| Web UI  | http://localhost:8080 | React app served by nginx |
| API     | http://localhost:3000 | `janmat-api` (also reachable at `:8080/api`) |
| DB      | localhost:5432 | Postgres 16 (user/pass/db = `janmat`) |

Override any port or DB credential via a `.env` file (see `.env.example`):

```bash
cp .env.example .env      # then edit WEB_PORT / API_PORT / POSTGRES_* etc.
docker compose up --build
```

Stop / reset:

```bash
docker compose down          # stop
docker compose down -v       # stop + wipe the DB volume
```

> The DB is real and used by the identity service (usernames + UIDs). If you
> run the API without a database, it automatically falls back to an in-memory
> registry so it still works.

---

## 2. Using the app

1. **Register.** Pick a **unique username** (checked live), then enter PAN,
   Aadhaar, DOB and phone. No external verification (POC). You receive a
   **16-character password** (save it) valid **24 hours**, cryptographically
   bound to all four fields.
2. **Log in with the password only.** A live **countdown** (top bar + profile)
   shows validity; you're **auto-signed-out at 24h**. Unlimited logins while
   valid. Each login **rotates your Citizen-UID seed** (a tamper-evident hash
   chain). After expiry, **"Forgot password"** re-mints one from the same four
   fields — no early reset.
3. **Communities → subtopics → opinions.** Pick or create a community (`j/…`)
   and a subtopic (`st/…`), then post an opinion, optionally with a **poll**.
4. **Vote** on any poll → results render as bars; your choice is highlighted.
   Expand a post's **receipt** to see the raw API response.
5. **Rules & voting** explains, in plain language, how decisions work and lets
   you suggest a change.

Communities/subtopics are seeded as empty starter structure; there is **no fake
post/vote data**. Your session persists in `localStorage`.

---

## 3. Local development (without Docker)

**API** (needs Rust stable, edition 2024):

```bash
cargo run -p janmat-api           # binds 127.0.0.1:3000, in-memory registry
# with Postgres:
DATABASE_URL=postgres://janmat:janmat@localhost:5432/janmat cargo run -p janmat-api
```

**Web** (needs Node 20+):

```bash
cd web
npm install
npm run dev                        # http://localhost:5173, proxies /api → :3000
```

The dev server proxies `/api` to `http://127.0.0.1:3000` (override with
`VITE_DEV_API_TARGET`).

---

## 4. API reference

Base URL: `http://localhost:3000` (or `http://localhost:8080/api` via the proxy).

| Method | Path | Purpose |
|--------|------|---------|
| GET  | `/health` | Liveness |
| GET  | `/citizen/kyc/health` | KYC subsystem health |
| GET  | `/citizen/username/{name}` | **Username availability** (pre-register check) |
| POST | `/citizen/register` | **POC registration** → returns a 16-char, 24h login password |
| POST | `/citizen/login` | **Login by password only** (rotates the Citizen-UID seed) |
| POST | `/citizen/password` | **Regenerate** a password after expiry (re-enter the 4 fields) |
| GET  | `/pub/constitution` | Constitution rules |
| GET  | `/pub/proposals` | List proposals (stub) |
| GET  | `/pub/t` | List topics (stub) |
| GET  | `/pub/t/{id}` | Get a topic |
| GET  | `/pub/t/{topic_id}/v/{vote_id}/results` | Vote results |
| POST | `/citizen/t` | Create topic |
| POST | `/citizen/o` | Submit opinion (content pipeline) |
| POST | `/citizen/v` | Create a vote/poll |
| POST | `/citizen/v/{id}/cast` | Cast a ZK-proved vote |
| POST | `/citizen/v/{id}/reveal` | Reveal toggle (202) |
| POST | `/citizen/proposals` | Create a proposal |
| POST | `/citizen/recall/{topic_id}` | Sign a recall petition |
| POST | `/citizen/kyc/upload` | Real UIDAI e-KYC upload (multipart) |

> **Path params take the bare ULID** (no `t/` / `v/` prefix), e.g.
> `/citizen/v/01KWY.../cast`.

---

## 5. Postman

Import both files from `postman/`:

- `JanMat.postman_collection.json` — every endpoint, each with **saved
  request + response examples** (incl. `409 username taken`, `404`, `400`).
- `JanMat.postman_environment.json` — sets `baseUrl`.

`Create topic` and `Create vote` auto-capture the returned ULID into
`{{topicId}}` / `{{voteId}}`, so the dependent path-param requests just work.

---

## 6. What was added / changed

**New**
- `web/` — Vite + React + TypeScript app (nginx Dockerfile + reverse proxy).
- `postman/` — collection + environment with Req/Res examples.
- `Dockerfile.api`, `web/Dockerfile`, `docker-compose.yml`, `.env.example`.
- `crates/janmat-api/src/db.rs` — Postgres/in-memory citizen registry.
- `crates/janmat-api/src/routes/register.rs` — register / login / username.

**Backend fixes (needed to make it run & vote work)**
- `main.rs`: Axum 0.8 path syntax `:id` → `{id}` (the server previously
  panicked on boot); bind address is now `JANMAT_BIND_ADDR` (0.0.0.0 in Docker).
- `pallet-voting`: added `open_vote(...)`; the cast handler now registers the
  ballot so a well-formed receipt is accepted (was always 422).
- `janmat-identity`: `poc_uid(...)` time-based identity hash.

---

## 7. Container security (docker scout)

Base images were hardened against reported CVEs:

| Image | Base | Fixable Critical/High |
|-------|------|----------------------|
| web | `nginx:1-alpine-slim` | **0** (was 4C / 23H on `nginx:*-alpine`) |
| api | `debian:13-slim`, runs as non-root | **0** |

Both images report **0 fixable vulnerabilities** via `docker scout cves
--only-fixed`. Any residual counts are unfixable-upstream Debian base findings
with no patch currently available.
