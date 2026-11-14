// Request/response shapes mirroring crates/janmat-api/src/routes/*.rs

export interface HealthResponse {
  status: string;
  service: string;
}

export interface KycHealthResponse {
  service: string;
  status: string;
  uidai_cert_loaded: boolean;
}

export interface RuleEntry {
  key: string;
  value: string;
  enacted_at_block: number;
  enacted_by_proposal: string;
}

export interface ConstitutionResponse {
  version: number;
  rules: RuleEntry[];
}

export interface CreateTopicRequest {
  slug: string;
}

export interface CreateTopicResponse {
  topic_id: string;
  anchor_hex: string;
}

export interface SubmitOpinionRequest {
  subtopic_id: string;
  content: string;
  creator_nullifier: string;
  clone_acknowledged: boolean;
}

export interface SubmitOpinionResponse {
  opinion_id: string;
  content_cid: string;
  content_hash: string;
  verdict: string;
  clone_candidates: string[];
}

export interface CreateVoteRequest {
  opinion_id: string;
  options: string[];
  closes_at_block: number;
}

export interface CreateVoteResponse {
  vote_id: string;
  options: string[];
  closes_at_block: number;
}

export interface ZkReceipt {
  circuit_id: string;
  proof_bytes: number[];
  public_outputs: number[];
}

export interface CastVoteRequest {
  zk_receipt: ZkReceipt;
}

export interface CastVoteResponse {
  accepted: boolean;
  nullifier_hex: string;
  commitment_hex: string;
  seq: number;
}

export interface VoteResultsResponse {
  vote_id: string;
  status: string;
  tally: unknown | null;
  attributed: unknown | null;
}

export interface CreateProposalRequest {
  proposal_type: "regular" | "constitutional";
  title: string;
  description: string;
  proposer_wallet: string;
}

export interface CreateProposalResponse {
  proposal_id: string;
  closes_at_block: number;
  required_quorum: number;
}

export interface RecallRequest {
  moderator_wallet: string;
  signer_wallet: string;
}

export interface RecallResponse {
  signatures: number;
  threshold: number;
  recall_vote_opened: boolean;
}

export interface KycVerifyResponse {
  success: boolean;
  nullifier_seed_commitment: string;
  citizenship_country: string;
  reference_id: string;
}

// ── Pulse & Consensus Ballot Types ──────────────────────────────

export interface AuthorMeta {
  name: string;
  role: string;
  verified: boolean;
  avatar_initials: string;
}

export interface BallotTally {
  agree_pct: number;
  oppose_pct: number;
  neutral_pct: number;
  agree_votes: number;
  oppose_votes: number;
  neutral_votes: number;
  total_ballots: number;
  quorum_pct: number;
}

export interface TopDissent {
  category: string;
  upvotes: number;
  quote: string;
}

export interface BallotItem {
  id: string;
  proposal_code: string;
  circle: string;
  circle_id: string;
  title: string;
  summary: string;
  author: AuthorMeta;
  status: "active" | "deliberation" | "passed" | "archived" | string;
  status_label: string;
  closes_in: string;
  category: string;
  tally: BallotTally;
  deliberation_count: number;
  blake3_hash: string;
  top_dissent?: TopDissent | null;
  created_at: string;
}

export interface FeaturedCircle {
  id: string;
  name: string;
  count: number;
  icon: string;
}

export interface VerificationReceipt {
  tx_id: string;
  action: string;
  block: string;
  delta: string;
  time_ago: string;
}

export interface TopNuancedDissent {
  topic: string;
  category: string;
  upvotes: number;
  quote: string;
  thread_link: string;
}

export interface PulseResponse {
  block_height: number;
  precinct_id: string;
  precinct_name: string;
  jurisdiction: string;
  status: string;
  network_label: string;
  turnout_pct: number;
  consensus_index: number;
  weekly_change: string;
  velocity: string;
  zk_verifier_health: string;
  blake3_anchor_hash: string;
  featured_circles: FeaturedCircle[];
  recent_receipts: VerificationReceipt[];
  top_dissent: TopNuancedDissent;
  timestamp: string;
}

export interface CastBallotResponse {
  accepted: boolean;
  ballot_id: string;
  choice: string;
  receipt_hash: string;
  block_height: number;
  timestamp: string;
  updated_tally: BallotTally;
}

export interface DraftMotionResponse {
  accepted: boolean;
  ballot: BallotItem;
}
