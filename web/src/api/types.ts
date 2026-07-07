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
