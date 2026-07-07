# Graph Report - .  (2026-06-27)

## Corpus Check
- Corpus is ~33,926 words - fits in a single context window. You may not need a graph.

## Summary
- 559 nodes · 1272 edges · 36 communities (33 shown, 3 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 25 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

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

## God Nodes (most connected - your core abstractions)
1. `GovernancePallet` - 31 edges
2. `make_pallet()` - 21 edges
3. `NamespacePallet` - 18 edges
4. `UidaiOfflineKycProvider` - 16 edges
5. `VoteRecord` - 15 edges
6. `Graphify Full Pipeline` - 15 edges
7. `JanMat Civic Democracy Platform` - 15 edges
8. `BlocklistEngine` - 14 edges
9. `Proposal` - 13 edges
10. `make_vote()` - 13 edges

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

## Communities (36 total, 3 thin omitted)

### Community 0 - "Governance Pallet & Proposals"
Cohesion: 0.07
Nodes (32): BTreeSet, make_pallet(), add_validator_requires_passed_proposal(), add_validator_without_passed_proposal_fails(), constitutional_requires_67_percent(), ConstitutionRule, ConstitutionVersion, double_vote_on_proposal_rejected() (+24 more)

### Community 1 - "KYC Identity & Credential Provider"
Cohesion: 0.09
Nodes (31): IntoResponse, KycCredential, KycProvider, MockKycProvider, nullifier_seed_commitment(), create_test_xml(), create_test_zip(), OfflineKycXml (+23 more)

### Community 2 - "HTTP API Routes & Handlers"
Cohesion: 0.10
Nodes (40): Json, Path, ConstitutionResponse, create_proposal(), CreateProposalRequest, CreateProposalResponse, get_constitution(), list_proposals() (+32 more)

### Community 3 - "CI Pipeline & Proto-Constitution"
Cohesion: 0.07
Nodes (40): CI Security Audit Job (cargo-audit), GitHub Actions CI Pipeline, CI Test Job (cargo fmt + clippy + test), Verified Citizen Civic Rights, Identity Proof Privacy Preservation, Immutable and Inspectable Governance Transitions, Procedural and Auditable Moderator Authority, JanMat Proto-Constitution (+32 more)

### Community 4 - "ZK Voting Pallet & Types"
Cohesion: 0.14
Nodes (24): OpinionId, AttributedVote, CastRecord, different_citizens_different_nullifiers(), double_cast_same_nullifier_is_revote_not_error(), full_zk_vote_cast_flow(), hex_bytes(), invalid_receipt_rejected() (+16 more)

### Community 5 - "Namespace & Topic Pallet"
Cohesion: 0.15
Nodes (21): make_pallet(), Hash, Result, create_topic_subtopic_opinion_chain(), NamespaceError, NamespacePallet, OpinionRecord, OutcomeRecord (+13 more)

### Community 6 - "Clone Detection Engine"
Cohesion: 0.14
Nodes (21): CloneDetector, CloneMatch, cosine_sim(), different_text_not_flagged(), identical_text_is_detected_as_clone(), IndexedOpinion, no_index_for_subtopic_returns_empty(), slightly_reworded_text_crosses_threshold() (+13 more)

### Community 7 - "ZK KYC Verification Circuits"
Cohesion: 0.13
Nodes (25): citizenship_hash(), derive_nullifier_seed(), empty_credential_rejected(), execute(), KycVerifyInput, KycVerifyOutput, non_indian_credential_rejected(), same_credential_same_nullifier_seed_commitment() (+17 more)

### Community 8 - "Internationalization (i18n)"
Cohesion: 0.15
Nodes (17): Default, TranslationKey, english_fallback_for_unseeded_language(), every_language_has_unique_code(), hindi_translation_returned(), Language, missing_key_returns_placeholder(), t() (+9 more)

### Community 9 - "Harm Filter & Content Safety"
Cohesion: 0.16
Nodes (18): coordination_keyword_triggers_block(), HarmDisposition, HarmFilter, HarmModel, HarmReport, HarmScore, normal_civic_text_passes(), single_hate_keyword_triggers_review() (+10 more)

### Community 10 - "Graphify Knowledge Graph Skill"
Cohesion: 0.10
Nodes (22): URL Ingest to Corpus (/graphify add), Graphify Skill Definition, MCP Server Export (--mcp), Confidence Score Rubric, Node ID Format Rules, Extraction Subagent Prompt Template, GitHub Repo Clone and Cross-Repo Merge, Post-Commit Auto-Rebuild Hook (+14 more)

### Community 11 - "Distributed Storage & IPFS"
Cohesion: 0.16
Nodes (15): Arc, Mutex, content_hash(), IpfsClient, IpfsObjectRef, MockIpfsClient, roundtrip_hash_verification(), tampered_content_fails_verification() (+7 more)

### Community 12 - "Moderation Pallet & Jury System"
Cohesion: 0.23
Nodes (13): BTreeMap, audit_log_is_append_only_and_immutable(), flag_creates_jury_and_audit_entry(), jury_selection_differs_with_different_block_hash(), jury_selection_is_deterministic(), jury_suppress_4_of_7_suppresses(), JurySession, JuryVerdict (+5 more)

### Community 13 - "Identity Pallet & Registration"
Cohesion: 0.18
Nodes (14): double_registration_same_credential_rejected(), full_kyc_registration_flow(), IdentityAnchor, IdentityError, IdentityPallet, invalid_proof_rejected(), register(), same_wallet_two_credentials_rejected() (+6 more)

### Community 14 - "Merkle Anchor & Vote Batching"
Cohesion: 0.18
Nodes (10): AnchorRecord, batch_votes_root(), batch_votes_root_anchored_correctly(), commitment_leaf(), empty_returns_known_constant(), hash_pair(), merkle_root(), MerkleAnchor (+2 more)

### Community 15 - "Blocklist & Phrase Filtering"
Cohesion: 0.23
Nodes (8): add_remove_creates_audit_trail(), BlocklistAction, BlocklistAuditEntry, BlocklistEngine, BlocklistEntry, clean_text_passes(), detects_blocked_phrase_case_insensitive(), duplicate_add_is_idempotent()

### Community 16 - "Rate Limiting Middleware"
Cohesion: 0.27
Nodes (4): InMemoryRateLimitStore, RateLimitAction, RateLimitStore, reset_clears_counts()

### Community 17 - "IVR Voice Voting Interface"
Cohesion: 0.44
Nodes (5): full_vote_flow(), history_records_visited_steps(), IvrSession, IvrStep, wrong_otp_stays_at_challenge()

### Community 18 - "ZK Nullifier & Commitment Scheme"
Cohesion: 0.36
Nodes (8): commitment_hides_choice(), derive_commitment(), derive_nullifier_seed(), derive_vote_nullifier(), different_citizens_different_seeds(), nullifier_seed_is_deterministic(), to_hex(), vote_nullifier_unique_per_vote()

### Community 19 - "Governance Integration Tests"
Cohesion: 0.60
Nodes (5): citizen_referenda_auto_triggers(), constitutional_amendment_full_flow(), moderator_recall_at_10_percent_opens_vote(), seeded(), validator_set_progressive_decentralisation()

## Knowledge Gaps
- **31 isolated node(s):** `KycUploadState`, `Graphify Skill Definition`, `AST Structural Extraction (Part A)`, `Semantic Extraction via Subagents (Part B)`, `Graph Build and Cluster Step` (+26 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `GovernancePallet` connect `Governance Pallet & Proposals` to `HTTP API Routes & Handlers`, `Governance Integration Tests`, `Moderation Pallet & Jury System`, `Clone Detection Engine`?**
  _High betweenness centrality (0.072) - this node is a cross-community bridge._
- **Why does `CheckReport` connect `Harm Filter & Content Safety` to `KYC Identity & Credential Provider`, `HTTP API Routes & Handlers`, `Clone Detection Engine`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `TranslationStore` connect `Internationalization (i18n)` to `HTTP API Routes & Handlers`, `Clone Detection Engine`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `KycUploadState`, `Graphify Skill Definition`, `AST Structural Extraction (Part A)` to the rest of the system?**
  _33 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Governance Pallet & Proposals` be split into smaller, more focused modules?**
  _Cohesion score 0.0697980684811238 - nodes in this community are weakly interconnected._
- **Should `KYC Identity & Credential Provider` be split into smaller, more focused modules?**
  _Cohesion score 0.09250693802035152 - nodes in this community are weakly interconnected._
- **Should `HTTP API Routes & Handlers` be split into smaller, more focused modules?**
  _Cohesion score 0.09528214616096208 - nodes in this community are weakly interconnected._