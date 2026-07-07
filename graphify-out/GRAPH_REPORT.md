# Graph Report - .  (2026-07-07)

## Corpus Check
- Corpus is ~49,893 words - fits in a single context window. You may not need a graph.

## Summary
- 777 nodes · 1743 edges · 45 communities (41 shown, 4 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 34 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_API Server & KYC Core|API Server & KYC Core]]
- [[_COMMUNITY_Governance & Validator|Governance & Validator]]
- [[_COMMUNITY_Namespace & Pallet Runtime|Namespace & Pallet Runtime]]
- [[_COMMUNITY_KYC Provider & Credentials|KYC Provider & Credentials]]
- [[_COMMUNITY_Blocklist & Moderation Engine|Blocklist & Moderation Engine]]
- [[_COMMUNITY_Indexer & Chain Moderation|Indexer & Chain Moderation]]
- [[_COMMUNITY_Constitution & Civil Rights|Constitution & Civil Rights]]
- [[_COMMUNITY_Voting Pallet & Types|Voting Pallet & Types]]
- [[_COMMUNITY_Frontend Components|Frontend Components]]
- [[_COMMUNITY_API Client & Identity|API Client & Identity]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]

## God Nodes (most connected - your core abstractions)
1. `GovernancePallet` - 31 edges
2. `useCivic()` - 24 edges
3. `make_pallet()` - 21 edges
4. `NamespacePallet` - 18 edges
5. `compilerOptions` - 18 edges
6. `register()` - 16 edges
7. `UidaiOfflineKycProvider` - 16 edges
8. `Registry` - 15 edges
9. `VoteRecord` - 15 edges
10. `Graphify Full Pipeline` - 15 edges

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
- **Graphify Extraction Pipeline (AST + Semantic + Merge)** — skills_skill_ast_extraction, skills_skill_semantic_extraction, skills_skill_extraction_cache, extraction_spec_subagent_prompt, skills_skill_graph_build [EXTRACTED 1.00]
- **JanMat ZK Privacy Stack** — readme_zk_identity_flow, readme_nullifier, readme_pallet_identity, readme_janmat_zk, idea_zkkeyc_ticketing, constitution_identity_privacy [INFERRED 0.85]
- **JanMat Governance and Moderation Layer** — readme_pallet_governance, readme_pallet_moderation, constitution_proto_constitution, idea_constitution_contract, constitution_supermajority, idea_moderator_protocol [INFERRED 0.85]
- **JanMat Three-Tier Application Stack** — docker_compose_db_service, docker_compose_api_service, docker_compose_web_service [EXTRACTED 1.00]
- **Release Pipeline Job Sequence (gate → docker+binary → release)** — workflows_release_ci_gate, workflows_release_docker_job, workflows_release_binary_job, workflows_release_release_job [EXTRACTED 1.00]
- **Citizen Interaction Flow (register → login → vote)** — quickstart_citizen_registration, quickstart_citizen_login, quickstart_zk_voting [INFERRED 0.85]

## Communities (45 total, 4 thin omitted)

### Community 0 - "API Server & KYC Core"
Cohesion: 0.06
Nodes (70): Arc, IntoResponse, Json, nullifier_seed_commitment(), UidaiError, Multipart, Path, ConstitutionResponse (+62 more)

### Community 1 - "Governance & Validator"
Cohesion: 0.06
Nodes (37): BTreeSet, make_pallet(), add_validator_requires_passed_proposal(), add_validator_without_passed_proposal_fails(), constitutional_requires_67_percent(), ConstitutionRule, ConstitutionVersion, double_vote_on_proposal_rejected() (+29 more)

### Community 2 - "Namespace & Pallet Runtime"
Cohesion: 0.10
Nodes (28): make_pallet(), Error, Hash, Option, Result, Row, Citizen, MemState (+20 more)

### Community 3 - "KYC Provider & Credentials"
Cohesion: 0.11
Nodes (24): KycCredential, KycProvider, MockKycProvider, create_test_xml(), create_test_zip(), OfflineKycXml, PoaXml, PoiXml (+16 more)

### Community 4 - "Blocklist & Moderation Engine"
Cohesion: 0.09
Nodes (26): add_remove_creates_audit_trail(), BlocklistAction, BlocklistAuditEntry, BlocklistEngine, BlocklistEntry, clean_text_passes(), detects_blocked_phrase_case_insensitive(), duplicate_add_is_idempotent() (+18 more)

### Community 5 - "Indexer & Chain Moderation"
Cohesion: 0.11
Nodes (23): BTreeMap, IndexerEvent, IndexReader, ingest_and_query_topics(), InMemoryIndexStore, moderation_log_filters_by_opinion(), OpinionRow, opinions_for_subtopic_returns_correct_subset() (+15 more)

### Community 6 - "Constitution & Civil Rights"
Cohesion: 0.08
Nodes (37): Verified Citizen Civic Rights, Identity Proof Privacy Preservation, Immutable and Inspectable Governance Transitions, Procedural and Auditable Moderator Authority, JanMat Proto-Constitution, Supermajority Quorum for Constitution Changes, Accessibility and Inclusion (SMS/IVR, Multi-Language), Civic Truth Machine Concept (+29 more)

### Community 7 - "Voting Pallet & Types"
Cohesion: 0.18
Nodes (22): OpinionId, AttributedVote, CastRecord, different_citizens_different_nullifiers(), double_cast_same_nullifier_is_revote_not_error(), full_zk_vote_cast_flow(), hex_bytes(), invalid_receipt_rejected() (+14 more)

### Community 8 - "Frontend Components"
Cohesion: 0.17
Nodes (18): api, UsernameAvailability, CommunityNav(), PostCard(), PostComposer(), Toasts(), TopBar(), Button() (+10 more)

### Community 9 - "API Client & Identity"
Cohesion: 0.12
Nodes (24): ApiError, LoginResponse, OpinionOutcome, parseBody(), RegenResponse, RegisterResponse, request(), CastVoteRequest (+16 more)

### Community 10 - "Community 10"
Cohesion: 0.15
Nodes (17): Default, TranslationKey, english_fallback_for_unseeded_language(), every_language_has_unique_code(), hindi_translation_returned(), Language, missing_key_returns_placeholder(), t() (+9 more)

### Community 11 - "Community 11"
Cohesion: 0.16
Nodes (15): CloneDetector, CloneMatch, cosine_sim(), different_text_not_flagged(), identical_text_is_detected_as_clone(), IndexedOpinion, no_index_for_subtopic_returns_empty(), slightly_reworded_text_crosses_threshold() (+7 more)

### Community 12 - "Community 12"
Cohesion: 0.14
Nodes (25): janmat-api Axum HTTP Service, PostgreSQL 16 DB Service, janmat-db Persistent Volume, Docker Compose Stack (three-tier), nginx + Vite/React SPA Web Service, API Reference Endpoints, Citizen Login (password-only + UID seed rotation), Citizen Registration Flow (PAN+Aadhaar+DOB+phone) (+17 more)

### Community 13 - "Community 13"
Cohesion: 0.09
Nodes (22): dependencies, @fontsource/ibm-plex-mono, @fontsource/space-grotesk, @fontsource-variable/inter, react, react-dom, description, devDependencies (+14 more)

### Community 14 - "Community 14"
Cohesion: 0.10
Nodes (22): URL Ingest to Corpus (/graphify add), Graphify Skill Definition, MCP Server Export (--mcp), Confidence Score Rubric, Node ID Format Rules, Extraction Subagent Prompt Template, GitHub Repo Clone and Cross-Repo Merge, Post-Commit Auto-Rebuild Hook (+14 more)

### Community 15 - "Community 15"
Cohesion: 0.18
Nodes (15): ZkReceipt, Poll(), Avatar(), CommunityTile(), PATHS, Receipt(), Variant, buildVoteReceipt() (+7 more)

### Community 16 - "Community 16"
Cohesion: 0.18
Nodes (14): double_registration_same_credential_rejected(), full_kyc_registration_flow(), IdentityAnchor, IdentityError, IdentityPallet, invalid_proof_rejected(), register(), same_wallet_two_credentials_rejected() (+6 more)

### Community 17 - "Community 17"
Cohesion: 0.17
Nodes (14): Mutex, content_hash(), IpfsClient, IpfsObjectRef, MockIpfsClient, roundtrip_hash_verification(), tampered_content_fails_verification(), verify_content_hash() (+6 more)

### Community 18 - "Community 18"
Cohesion: 0.10
Nodes (19): compilerOptions, allowImportingTsExtensions, isolatedModules, jsx, lib, module, moduleDetection, moduleResolution (+11 more)

### Community 19 - "Community 19"
Cohesion: 0.20
Nodes (15): citizenship_hash(), derive_nullifier_seed(), empty_credential_rejected(), execute(), KycVerifyInput, KycVerifyOutput, non_indian_credential_rejected(), same_credential_same_nullifier_seed_commitment() (+7 more)

### Community 20 - "Community 20"
Cohesion: 0.18
Nodes (10): AnchorRecord, batch_votes_root(), batch_votes_root_anchored_correctly(), commitment_leaf(), empty_returns_known_constant(), hash_pair(), merkle_root(), MerkleAnchor (+2 more)

### Community 21 - "Community 21"
Cohesion: 0.13
Nodes (13): RegenRequest, RegisterRequest, Action, Ctx, Identity, initialState(), Poll, seedTopics() (+5 more)

### Community 22 - "Community 22"
Cohesion: 0.22
Nodes (10): CreateProposalResponse, AboutRail(), timeAgo(), friendlyRule(), META, RuleView, Unit, CHANGE_TYPES (+2 more)

### Community 23 - "Community 23"
Cohesion: 0.40
Nodes (10): derive_commitment(), derive_vote_nullifier(), different_choice_same_nullifier_different_commitment(), different_vote_id_gives_different_nullifier(), empty_seed_rejected(), execute(), produces_deterministic_outputs(), valid_input() (+2 more)

### Community 24 - "Community 24"
Cohesion: 0.44
Nodes (5): full_vote_flow(), history_records_visited_steps(), IvrSession, IvrStep, wrong_otp_stays_at_challenge()

### Community 25 - "Community 25"
Cohesion: 0.36
Nodes (8): commitment_hides_choice(), derive_commitment(), derive_nullifier_seed(), derive_vote_nullifier(), different_citizens_different_seeds(), nullifier_seed_is_deterministic(), to_hex(), vote_nullifier_unique_per_vote()

### Community 26 - "Community 26"
Cohesion: 0.38
Nodes (3): MockVerifier, ZkReceipt, ZkVerifier

## Knowledge Gaps
- **92 isolated node(s):** `KycUploadState`, `name`, `private`, `version`, `type` (+87 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `State` connect `API Server & KYC Core` to `Community 21`?**
  _High betweenness centrality (0.183) - this node is a cross-community bridge._
- **Why does `username_available()` connect `API Server & KYC Core` to `Namespace & Pallet Runtime`?**
  _High betweenness centrality (0.137) - this node is a cross-community bridge._
- **Why does `GovernancePallet` connect `Governance & Validator` to `API Server & KYC Core`, `Indexer & Chain Moderation`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **What connects `KycUploadState`, `name`, `private` to the rest of the system?**
  _94 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `API Server & KYC Core` be split into smaller, more focused modules?**
  _Cohesion score 0.06101914962674456 - nodes in this community are weakly interconnected._
- **Should `Governance & Validator` be split into smaller, more focused modules?**
  _Cohesion score 0.06293965198074787 - nodes in this community are weakly interconnected._
- **Should `Namespace & Pallet Runtime` be split into smaller, more focused modules?**
  _Cohesion score 0.09803921568627451 - nodes in this community are weakly interconnected._