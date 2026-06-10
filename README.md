# JanMat — People's Opinion

> **A Rust-first, blockchain-anchored civic democracy platform for India.**
> Tamper-proof. Privacy-preserving. People-owned.

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

---

## What is JanMat?

JanMat (जनमत — *People's Opinion*) is a minimal, immutable civic ledger. Citizens prove they are Indian via zero-knowledge credentials — their Aadhaar number **never enters the system**. They create structured civic topics, submit opinions, and cast verifiably anonymous votes. Every action is chain-anchored, auditable, and censorship-resistant.

**It is not a social network.** It is a constitutional machine.

---

## Core Properties

| Property | How it is enforced |
|---|---|
| **One citizen, one identity** | ZK nullifier derived from Aadhaar credential — same person cannot register twice |
| **One vote per ballot** | Per-vote nullifier stored on-chain; `pallet-voting` rejects duplicates |
| **Anonymous by default** | Votes stored as Blake3 commitments; tally runs without revealing choices |
| **Irreversible reveal** | Creator can toggle visibility to Public exactly once — cannot be undone |
| **Immutable opinions** | Opinions are content-hash-locked on submission; status changes only |
| **No secret removals** | Suppression requires a 4-of-7 citizen jury vote; every action is on-chain |
| **Governed rules** | Constitution itself is a chain contract; changes need 67% supermajority vote |

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Layer 6 — Frontend (Leptos / WASM)                          │
│  Topic Browser · Opinion + Vote UI · zkKYC Onboarding        │
├──────────────────────────────────────────────────────────────┤
│  Layer 5 — API (Axum)                  janmat-api            │
│  /pub/* read · /citizen/* write · WS live feed               │
├──────────────────────────────────────────────────────────────┤
│  Layer 4 — Content Safety              janmat-content        │
│  Blocklist Engine · Harm Filter · Clone Detector             │
├──────────────────────────────────────────────────────────────┤
│  Layer 3 — ZK Proof System             janmat-zk             │
│  kyc-verify circuit · vote-cast circuit · MockProver (→ RISC Zero) │
├──────────────────────────────────────────────────────────────┤
│  Layer 2 — Identity                    janmat-identity       │
│  KycCredential · MockKycProvider · Nullifier helpers         │
├──────────────────────────────────────────────────────────────┤
│  Layer 1 — Blockchain                  janmat-chain          │
│  pallet-namespace · pallet-voting · pallet-identity          │
│  pallet-moderation · pallet-governance                       │
├──────────────────────────────────────────────────────────────┤
│  Layer 0 — Storage                     janmat-storage        │
│  IPFS content · Blake3 integrity · Chain indexer (pg)        │
└──────────────────────────────────────────────────────────────┘
```

---

## Namespace Design

Every entity is addressed with a typed ULID:

```
t/  — Topic          e.g.  t/01HXYZ...   "education-nep-2026"
st/ — Subtopic       e.g.  st/01HABC...  "nep-rural-implementation"
o/  — Opinion        e.g.  o/01HDEF...   (IPFS CID + Blake3 hash)
v/  — Vote           e.g.  v/01HGHI...   (options, tally, visibility)
```

Every ULID is block-hash anchored at creation: `anchor = Blake3(ulid_bytes ++ block_hash)` — backdating is impossible.

---

## ZK Identity Flow

```
Citizen → KYC Provider (UIDAI-licensed)
       ← Signed ZK Credential  { citizenship: India, salt: H(aadhaar||salt) }
                                  [Aadhaar number NEVER sent to JanMat]
Citizen → janmat-zk  → kyc_proof (ZkReceipt)
       → pallet-identity.register_wallet(kyc_proof)
       ← on-chain identity anchor  { nullifier_seed_commitment, wallet }
```

The chain stores **only** the `nullifier_seed_commitment = Blake3(Blake3(cred||salt))`. No Aadhaar data at any layer.

---

## Vote Lifecycle

```
1. CREATE   create_vote(opinion_id, options, closes_at_block)
2. CAST     cast_vote(vote_id, zk_proof, commitment, nullifier)
               → pallet checks: valid receipt + nullifier not used
3. CLOSE    close_tally(vote_id) at closes_at_block
               → commitments counted, tally written on-chain
4. REVEAL   reveal_toggle(vote_id)   [one-way, creator only]
               → Phase 4: threshold ElGamal decrypt of enc_attr bundles
               → each voter's choice becomes publicly attributable
```

---

## Workspace Structure

```
JanMat/
├── Cargo.toml                        Workspace root
├── crates/
│   ├── janmat-types/                 Shared primitives (ULID, IDs, enums)
│   ├── janmat-zk/                    ZK circuits + MockProver
│   │   ├── src/circuits/
│   │   │   ├── kyc_verify.rs         Citizenship proof circuit
│   │   │   └── vote_cast.rs          Vote nullifier + commitment circuit
│   │   └── src/prover/mock.rs        RISC Zero plug-in ready prover
│   ├── janmat-identity/              KYC credential bridge + nullifier helpers
│   ├── janmat-chain/
│   │   └── pallets/
│   │       ├── pallet-namespace/     t/st/o/v creation, immutability
│   │       ├── pallet-voting/        Ballot engine, nullifier registry, reveal
│   │       ├── pallet-identity/      zkKYC anchor, double-reg prevention
│   │       ├── pallet-moderation/    Citizen jury, audit log
│   │       └── pallet-governance/   Constitution, quorum math
│   ├── janmat-content/               Content safety pipeline
│   │   ├── src/blocklist/            Governed phrase list
│   │   ├── src/harm_filter/          Category scoring (→ candle ONNX)
│   │   ├── src/clone_detector/       TF-IDF cosine similarity
│   │   └── src/pipeline.rs           SubmitRequest → CheckReport
│   ├── janmat-storage/               IpfsClient trait + MockIpfsClient
│   ├── janmat-api/                   Axum API server
│   │   └── src/routes/
│   │       ├── namespace.rs          GET/POST /t
│   │       ├── opinions.rs           POST /citizen/o
│   │       └── votes.rs              POST/GET /v
│   └── janmat-governance/            Quorum helpers
├── frontend/                         Leptos WASM app (Phase 6)
└── docs/
    └── constitution.md               Seed proto-constitution
```

---

## Development Phases

| Phase | Status | Scope |
|---|---|---|
| **0** | ✅ Done | Workspace scaffold, shared types, ULID anchoring |
| **1** | ✅ Done | Chain core — namespace, voting, identity, storage, API routes |
| **2** | ✅ Done | ZK identity — kyc-verify + vote-cast circuits, MockProver, pallet-identity upgrade |
| **3** | ✅ Done | Content safety — BlocklistEngine, HarmFilter, CloneDetector, pallet-moderation |
| **4** | 🚧 Next | Full ZK voting — commitment-reveal, pallet-voting ZK wiring, re-vote resistance |
| **5** | ⏳ | Governance layer — constitution contract, moderator elections, recall |
| **6** | ⏳ | Production hardening — 22-language UI, SMS/IVR, ZK rollup, security audit |

---

## Running Tests

```bash
cargo test --workspace
```

All tests are pure Rust — no external services, no Docker, no network.

## Running the API

```bash
cargo run -p janmat-api
# → janmat-api listening on 127.0.0.1:3000

curl http://localhost:3000/health
# → {"status":"ok","service":"janmat-api"}
```

---

## Key Design Decisions

**Substrate over Solana** — full pallet customisation, Rust-native `ink!`, flexible validator governance. Solana's centralised validator set is incompatible with civic sovereignty.

**RISC Zero over circom** — pure Rust circuits, no JavaScript/circom dependency, WASM prover runs in-browser without plugins.

**Blake3 over SHA-256** — faster, parallelisable, Rust-native, BLAKE3 is resistant to length-extension attacks.

**No shadow-banning** — every suppression has a block hash, a juror list, and a reason code. The record is permanent.

**Reversibility is coercion** — the reveal toggle is one-way by design. A politician cannot flip visibility on/off to intimidate voters retroactively.

---

## Legal Note

Aadhaar API integration requires a UIDAI-licensed KYC provider. Phases 0–5 use `MockKycProvider`. Real UIDAI integration is a compliance track running parallel to technical development. Engage UIDAI + legal counsel before Phase 6 production deployment.

---

## License

MIT — see [LICENSE](LICENSE).
