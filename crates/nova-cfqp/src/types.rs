use nova_types::Digest;
use std::collections::BTreeSet;
use std::fmt;

// PHASE 1A Identity Types

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EvidenceId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DecisionId(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutcomeId(pub u64);

// Timestamps: milliseconds since epoch
pub type Timestamp = u64;

// Information Boundary
// Enforces temporal epistemic boundaries: evidence available at decision time

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InformationBoundary {
    /// The decision point in time
    pub decision_time: Timestamp,
}

impl InformationBoundary {
    pub fn new(decision_time: Timestamp) -> Self {
        Self { decision_time }
    }

    /// Check if evidence is admissible at this boundary
    /// Core invariant: available_at <= decision_time
    pub fn admits(&self, available_at: Timestamp) -> bool {
        available_at <= self.decision_time
    }
}

// Evidence: observed fact with temporal and availability semantics

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub id: EvidenceId,
    /// Timestamp when evidence was observed
    pub observed_at: Timestamp,
    /// Timestamp when evidence became available for decision-making
    pub available_at: Timestamp,
    /// The content of the evidence
    pub content: Vec<u8>,
    /// Cryptographic commitment to this evidence
    pub digest: Digest,
}

impl Evidence {
    pub fn new(id: EvidenceId, observed_at: Timestamp, available_at: Timestamp, content: Vec<u8>) -> Self {
        let mut canonical = Vec::new();
        canonical.extend_from_slice(&id.0.to_le_bytes());
        canonical.extend_from_slice(&observed_at.to_le_bytes());
        canonical.extend_from_slice(&available_at.to_le_bytes());
        canonical.extend_from_slice(&(content.len() as u64).to_le_bytes());
        canonical.extend_from_slice(&content);
        let digest = Digest::of(crate::domains::EVIDENCE_DOMAIN, &canonical);
        Self {
            id,
            observed_at,
            available_at,
            content,
            digest,
        }
    }
}

// InformationSet: deterministic, timestamped collection of evidence
// Semantics: membership set (unordered), but canonical commitment is deterministic

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InformationSet {
    pub boundary: InformationBoundary,
    /// Evidence included in this set
    /// Stored as sorted for deterministic canonical representation
    evidence_ids: BTreeSet<EvidenceId>,
    evidence: Vec<Evidence>,
    /// Commitment to the entire set
    pub set_digest: Digest,
}

impl InformationSet {
    pub fn new(boundary: InformationBoundary, evidence: Vec<Evidence>) -> Result<Self, String> {
        // Verify that all evidence is admissible at the boundary
        for e in &evidence {
            if !boundary.admits(e.available_at) {
                return Err(format!(
                    "Evidence {} available at {} exceeds boundary {}",
                    e.id.0, e.available_at, boundary.decision_time
                ));
            }
        }

        // Canonical ordering: sorted by ID for deterministic serialization
        let mut sorted = evidence;
        sorted.sort_by_key(|e| e.id);

        let ids: BTreeSet<EvidenceId> = sorted.iter().map(|e| e.id).collect();

        // Compute canonical commitment
        let mut canonical = Vec::new();
        canonical.extend_from_slice(&boundary.decision_time.to_le_bytes());
        for e in &sorted {
            canonical.extend_from_slice(&e.id.0.to_le_bytes());
            canonical.extend_from_slice(e.digest.as_bytes());
        }

        let set_digest = Digest::of(crate::domains::INFORMATION_SET_DOMAIN, &canonical);

        Ok(Self {
            boundary,
            evidence_ids: ids,
            evidence: sorted,
            set_digest,
        })
    }

    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    pub fn contains(&self, id: EvidenceId) -> bool {
        self.evidence_ids.contains(&id)
    }
}

// Decision: action taken under a historical information boundary

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decision {
    pub id: DecisionId,
    /// Actor who made the decision
    pub actor: u64, // placeholder; in full system would be AgentId
    /// The information set available at decision time
    pub information_set: InformationSet,
    /// The action / decision to take
    pub action: Vec<u8>,
    /// Commitment to the decision
    pub digest: Digest,
}

impl Decision {
    pub fn new(id: DecisionId, actor: u64, information_set: InformationSet, action: Vec<u8>) -> Self {
        // Canonical commitment binds decision to its historical information set
        let mut canonical = Vec::new();
        canonical.extend_from_slice(&id.0.to_le_bytes());
        canonical.extend_from_slice(&actor.to_le_bytes());
        canonical.extend_from_slice(information_set.set_digest.as_bytes());
        canonical.extend_from_slice(&action);

        let digest = Digest::of(crate::domains::DECISION_DOMAIN, &canonical);

        Self {
            id,
            actor,
            information_set,
            action,
            digest,
        }
    }
}

// Outcome: what actually happened after the decision
// Temporally distinct from Decision

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome {
    pub id: OutcomeId,
    /// Decision this outcome corresponds to
    pub decision_id: DecisionId,
    /// When the outcome occurred
    pub occurred_at: Timestamp,
    /// The observed result
    pub result: Vec<u8>,
    /// Commitment to the outcome
    pub digest: Digest,
}

impl Outcome {
    pub fn new(id: OutcomeId, decision_id: DecisionId, occurred_at: Timestamp, result: Vec<u8>) -> Self {
        // Outcome commitment does NOT include the decision's information set
        // This prevents outcome from retroactively altering decision history
        let mut canonical = Vec::new();
        canonical.extend_from_slice(&id.0.to_le_bytes());
        canonical.extend_from_slice(&decision_id.0.to_le_bytes());
        canonical.extend_from_slice(&occurred_at.to_le_bytes());
        canonical.extend_from_slice(&result);

        let digest = Digest::of(crate::domains::OUTCOME_DOMAIN, &canonical);

        Self {
            id,
            decision_id,
            occurred_at,
            result,
            digest,
        }
    }
}
