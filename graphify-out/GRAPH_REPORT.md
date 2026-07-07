# Graph Report - JanMat  (2026-07-07)

## Corpus Check
- 93 files · ~46,138 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 812 nodes · 1708 edges · 53 communities (43 shown, 10 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d3aaf6ad`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Governance Pallet & Proposals|Governance Pallet & Proposals]]
- [[_COMMUNITY_KYC Identity & Credential Provider|KYC Identity & Credential Provider]]
- [[_COMMUNITY_HTTP API Routes & Handlers|HTTP API Routes & Handlers]]
- [[_COMMUNITY_CI Pipeline & Proto-Constitution|CI Pipeline & Proto-Constitution]]
- [[_COMMUNITY_ZK Voting Pallet & Types|ZK Voting Pallet & Types]]
- [[_COMMUNITY_Namespace & Topic Pallet|Namespace & Topic Pallet]]
- [[_COMMUNITY_Clone Detection Engine|Clone Detection Engine]]
- [[_COMMUNITY_ZK KYC Verification Circuits|ZK KYC Verification Circuits]]
- [[_COMMUNITY_Internationalization (i18n)|Internationalization (i18n)]]
- [[_COMMUNITY_Harm Filter & Content Safety|Harm Filter & Content Safety]]
- [[_COMMUNITY_Graphify Knowledge Graph Skill|Graphify Knowledge Graph Skill]]
- [[_COMMUNITY_Distributed Storage & IPFS|Distributed Storage & IPFS]]
- [[_COMMUNITY_Moderation Pallet & Jury System|Moderation Pallet & Jury System]]
- [[_COMMUNITY_Identity Pallet & Registration|Identity Pallet & Registration]]
- [[_COMMUNITY_Merkle Anchor & Vote Batching|Merkle Anchor & Vote Batching]]
- [[_COMMUNITY_Blocklist & Phrase Filtering|Blocklist & Phrase Filtering]]
- [[_COMMUNITY_Rate Limiting Middleware|Rate Limiting Middleware]]
- [[_COMMUNITY_IVR Voice Voting Interface|IVR Voice Voting Interface]]
- [[_COMMUNITY_ZK Nullifier & Commitment Scheme|ZK Nullifier & Commitment Scheme]]
- [[_COMMUNITY_Governance Integration Tests|Governance Integration Tests]]
- [[_COMMUNITY_Graphify Watch Mode|Graphify Watch Mode]]
- [[_COMMUNITY_Graphify Neo4j Export|Graphify Neo4j Export]]
- [[_COMMUNITY_Graphify Wiki Export|Graphify Wiki Export]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 48|Community 48]]
- [[_COMMUNITY_Community 49|Community 49]]
- [[_COMMUNITY_Community 50|Community 50]]
- [[_COMMUNITY_Community 51|Community 51]]

## God Nodes (most connected - your core abstractions)
1. `GovernancePallet` - 31 edges
2. `useCivic()` - 23 edges
3. `make_pallet()` - 21 edges
4. `NamespacePallet` - 18 edges
5. `compilerOptions` - 18 edges
6. `register()` - 16 edges
7. `UidaiOfflineKycProvider` - 16 edges
8. `VoteRecord` - 15 edges
9. `Graphify Full Pipeline` - 15 edges
10. `JanMat Civic Democracy Platform` - 15 edges

## Surprising Connections (you probably didn't know these)
- `Identity Proof Privacy Preservation` --semantically_similar_to--> `ZK Identity Flow`  [INFERRED] [semantically similar]
  docs/constitution.md → README.md
- `zkKYC + zkVoting Identity Ticketing` --semantically_similar_to--> `ZK Identity Flow`  [INFERRED] [semantically similar]
  idea.txt → README.md
- `Harm Filter Design (Vectors of Harm vs Topics)` --semantically_similar_to--> `janmat-content (Content Safety Pipeline)`  [INFERRED] [semantically similar]
  idea.txt → README.md
- `Moderator Election Protocol (Term-Limited, Zero Unilateral Power)` --semantically_similar_to--> `pallet-moderation (Citizen Jury)`  [INFERRED] [semantically similar]
  idea.txt → README.md
- `Constitution Contract (Meta-Governance Layer)` --semantically_similar_to--> `pallet-governance (Constitution Contract)`  [INFERRED] [semantically similar]
  idea.txt → README.md

## Import Cycles
- 1-file cycle: `crates/janmat-types/src/ulid.rs -> crates/janmat-types/src/ulid.rs`

## Hyperedges (group relationships)
- **JanMat ZK Privacy Stack** — readme_zk_identity_flow, readme_nullifier, readme_pallet_identity, readme_janmat_zk, idea_zkkeyc_ticketing, constitution_identity_privacy [INFERRED 0.85]
- **JanMat Governance and Moderation Layer** — readme_pallet_governance, readme_pallet_moderation, constitution_proto_constitution, idea_constitution_contract, constitution_supermajority, idea_moderator_protocol [INFERRED 0.85]
- **Graphify Extraction Pipeline (AST + Semantic + Merge)** — skills_skill_ast_extraction, skills_skill_semantic_extraction, skills_skill_extraction_cache, extraction_spec_subagent_prompt, skills_skill_graph_build [EXTRACTED 1.00]

## Communities (53 total, 10 thin omitted)

### Community 0 - "Governance Pallet & Proposals"
Cohesion: 0.07
Nodes (32): BTreeSet, make_pallet(), add_validator_requires_passed_proposal(), add_validator_without_passed_proposal_fails(), constitutional_requires_67_percent(), ConstitutionRule, ConstitutionVersion, double_vote_on_proposal_rejected() (+24 more)

### Community 1 - "KYC Identity & Credential Provider"
Cohesion: 0.08
Nodes (34): IntoResponse, KycCredential, KycProvider, MockKycProvider, nullifier_seed_commitment(), poc_uid(), create_test_xml(), create_test_zip() (+26 more)

### Community 2 - "HTTP API Routes & Handlers"
Cohesion: 0.07
Nodes (59): Arc, Error, Json, Path, ConstitutionResponse, create_proposal(), CreateProposalRequest, CreateProposalResponse (+51 more)

### Community 3 - "CI Pipeline & Proto-Constitution"
Cohesion: 0.05
Nodes (50): CI Security Audit Job (cargo-audit), GitHub Actions CI Pipeline, CI Test Job (cargo fmt + clippy + test), Verified Citizen Civic Rights, Identity Proof Privacy Preservation, Immutable and Inspectable Governance Transitions, Procedural and Auditable Moderator Authority, JanMat Proto-Constitution (+42 more)

### Community 4 - "ZK Voting Pallet & Types"
Cohesion: 0.10
Nodes (30): OpinionId, kyc_input(), MockProver, prove_kyc_non_india_fails(), prove_kyc_produces_receipt(), prove_vote_produces_receipt(), AttributedVote, CastRecord (+22 more)

### Community 5 - "Namespace & Topic Pallet"
Cohesion: 0.15
Nodes (21): make_pallet(), Hash, Result, create_topic_subtopic_opinion_chain(), NamespaceError, NamespacePallet, OpinionRecord, OutcomeRecord (+13 more)

### Community 6 - "Clone Detection Engine"
Cohesion: 0.16
Nodes (15): CloneDetector, CloneMatch, cosine_sim(), different_text_not_flagged(), identical_text_is_detected_as_clone(), IndexedOpinion, no_index_for_subtopic_returns_empty(), slightly_reworded_text_crosses_threshold() (+7 more)

### Community 7 - "ZK KYC Verification Circuits"
Cohesion: 0.38
Nodes (10): citizenship_hash(), derive_nullifier_seed(), empty_credential_rejected(), execute(), KycVerifyInput, KycVerifyOutput, non_indian_credential_rejected(), same_credential_same_nullifier_seed_commitment() (+2 more)

### Community 8 - "Internationalization (i18n)"
Cohesion: 0.15
Nodes (17): Default, TranslationKey, english_fallback_for_unseeded_language(), every_language_has_unique_code(), hindi_translation_returned(), Language, missing_key_returns_placeholder(), t() (+9 more)

### Community 9 - "Harm Filter & Content Safety"
Cohesion: 0.09
Nodes (26): add_remove_creates_audit_trail(), BlocklistAction, BlocklistAuditEntry, BlocklistEngine, BlocklistEntry, clean_text_passes(), detects_blocked_phrase_case_insensitive(), duplicate_add_is_idempotent() (+18 more)

### Community 10 - "Graphify Knowledge Graph Skill"
Cohesion: 0.10
Nodes (22): URL Ingest to Corpus (/graphify add), Graphify Skill Definition, MCP Server Export (--mcp), Confidence Score Rubric, Node ID Format Rules, Extraction Subagent Prompt Template, GitHub Repo Clone and Cross-Repo Merge, Post-Commit Auto-Rebuild Hook (+14 more)

### Community 11 - "Distributed Storage & IPFS"
Cohesion: 0.17
Nodes (14): Mutex, content_hash(), IpfsClient, IpfsObjectRef, MockIpfsClient, roundtrip_hash_verification(), tampered_content_fails_verification(), verify_content_hash() (+6 more)

### Community 12 - "Moderation Pallet & Jury System"
Cohesion: 0.12
Nodes (24): BTreeMap, Option, IndexerEvent, IndexReader, ingest_and_query_topics(), InMemoryIndexStore, moderation_log_filters_by_opinion(), OpinionRow (+16 more)

### Community 13 - "Identity Pallet & Registration"
Cohesion: 0.18
Nodes (14): double_registration_same_credential_rejected(), full_kyc_registration_flow(), IdentityAnchor, IdentityError, IdentityPallet, invalid_proof_rejected(), register(), same_wallet_two_credentials_rejected() (+6 more)

### Community 14 - "Merkle Anchor & Vote Batching"
Cohesion: 0.18
Nodes (10): AnchorRecord, batch_votes_root(), batch_votes_root_anchored_correctly(), commitment_leaf(), empty_returns_known_constant(), hash_pair(), merkle_root(), MerkleAnchor (+2 more)

### Community 15 - "Blocklist & Phrase Filtering"
Cohesion: 0.05
Nodes (70): api, ApiError, LoginResponse, OpinionOutcome, parseBody(), RegisterRequest, RegisterResponse, request() (+62 more)

### Community 16 - "Rate Limiting Middleware"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 17 - "IVR Voice Voting Interface"
Cohesion: 0.44
Nodes (5): full_vote_flow(), history_records_visited_steps(), IvrSession, IvrStep, wrong_otp_stays_at_challenge()

### Community 18 - "ZK Nullifier & Commitment Scheme"
Cohesion: 0.36
Nodes (8): commitment_hides_choice(), derive_commitment(), derive_nullifier_seed(), derive_vote_nullifier(), different_citizens_different_seeds(), nullifier_seed_is_deterministic(), to_hex(), vote_nullifier_unique_per_vote()

### Community 19 - "Governance Integration Tests"
Cohesion: 0.60
Nodes (5): citizen_referenda_auto_triggers(), constitutional_amendment_full_flow(), moderator_recall_at_10_percent_opens_vote(), seeded(), validator_set_progressive_decentralisation()

### Community 36 - "Community 36"
Cohesion: 0.10
Nodes (19): dependencies, react, react-dom, description, devDependencies, @types/node, @types/react, @types/react-dom (+11 more)

### Community 37 - "Community 37"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, isolatedModules, jsx, lib, module, moduleDetection, moduleResolution (+11 more)

### Community 38 - "Community 38"
Cohesion: 0.40
Nodes (10): derive_commitment(), derive_vote_nullifier(), different_choice_same_nullifier_different_commitment(), different_vote_id_gives_different_nullifier(), empty_seed_rejected(), execute(), produces_deterministic_outputs(), valid_input() (+2 more)

### Community 39 - "Community 39"
Cohesion: 0.22
Nodes (8): 1. Run everything (one command), 2. Using the app, 3. Local development (without Docker), 4. API reference, 5. Postman, 6. What was added / changed, 7. Container security (docker scout), JanMat — Quickstart (Web UI · API · DB · Postman)

### Community 40 - "Community 40"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 41 - "Community 41"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 42 - "Community 42"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 43 - "Community 43"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 44 - "Community 44"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

## Knowledge Gaps
- **138 isolated node(s):** `KycUploadState`, `name`, `private`, `version`, `type` (+133 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **10 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `State` connect `HTTP API Routes & Handlers` to `Blocklist & Phrase Filtering`?**
  _High betweenness centrality (0.142) - this node is a cross-community bridge._
- **Why does `username_available()` connect `HTTP API Routes & Handlers` to `Namespace & Topic Pallet`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Why does `GovernancePallet` connect `Governance Pallet & Proposals` to `HTTP API Routes & Handlers`, `Governance Integration Tests`, `Moderation Pallet & Jury System`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `KycUploadState`, `name`, `private` to the rest of the system?**
  _139 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Governance Pallet & Proposals` be split into smaller, more focused modules?**
  _Cohesion score 0.0697980684811238 - nodes in this community are weakly interconnected._
- **Should `KYC Identity & Credential Provider` be split into smaller, more focused modules?**
  _Cohesion score 0.07767722473604827 - nodes in this community are weakly interconnected._
- **Should `HTTP API Routes & Handlers` be split into smaller, more focused modules?**
  _Cohesion score 0.06961770623742455 - nodes in this community are weakly interconnected._