use nova_types::{Digest, StateRoot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionProposal {
    pub id: u64,
    pub state_root: StateRoot,
    pub objective: String,
    pub evidence: Vec<Digest>,
    pub action: String,
    pub expected_outcome: String,
    pub uncertainty_bps: u16,
    pub expiry_epoch: u64,
}

impl DecisionProposal {
    pub fn digest(&self) -> Digest {
        let mut b = Vec::new();
        b.extend_from_slice(&self.id.to_le_bytes());
        b.extend_from_slice(&self.state_root.0);
        for s in [&self.objective, &self.action, &self.expected_outcome] {
            b.extend_from_slice(&(s.len() as u64).to_le_bytes());
            b.extend_from_slice(s.as_bytes());
        }
        for e in &self.evidence { b.extend_from_slice(&e.0); }
        b.extend_from_slice(&self.uncertainty_bps.to_le_bytes());
        b.extend_from_slice(&self.expiry_epoch.to_le_bytes());
        Digest::of(b"nova.decision_proposal.v1", &b)
    }

    pub fn validate(&self, current_root: StateRoot, epoch: u64) -> Result<(), DecisionError> {
        if self.state_root != current_root { return Err(DecisionError::StaleStateRoot); }
        if self.objective.is_empty() || self.action.is_empty() || self.expected_outcome.is_empty() {
            return Err(DecisionError::IncompleteProposal);
        }
        if self.evidence.is_empty() { return Err(DecisionError::MissingEvidence); }
        if self.uncertainty_bps > 10_000 { return Err(DecisionError::InvalidUncertainty); }
        if epoch > self.expiry_epoch { return Err(DecisionError::Expired); }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecisionError { StaleStateRoot, IncompleteProposal, MissingEvidence, InvalidUncertainty, Expired }

#[cfg(test)]
mod tests {
    use super::*;
    fn proposal(root: StateRoot) -> DecisionProposal {
        DecisionProposal { id: 1, state_root: root, objective: "preserve state".into(), evidence: vec![Digest::of(b"e", b"1")], action: "observe".into(), expected_outcome: "new evidence".into(), uncertainty_bps: 2000, expiry_epoch: 10 }
    }
    #[test] fn stale_root_is_rejected() { assert_eq!(proposal(StateRoot([1;32])).validate(StateRoot([2;32]), 1), Err(DecisionError::StaleStateRoot)); }
    #[test] fn missing_evidence_is_rejected() { let mut p=proposal(StateRoot([1;32])); p.evidence.clear(); assert_eq!(p.validate(StateRoot([1;32]),1), Err(DecisionError::MissingEvidence)); }
    #[test] fn expired_proposal_is_rejected() { assert_eq!(proposal(StateRoot([1;32])).validate(StateRoot([1;32]),11), Err(DecisionError::Expired)); }
    #[test] fn identical_proposals_have_identical_digests() { assert_eq!(proposal(StateRoot([1;32])).digest(), proposal(StateRoot([1;32])).digest()); }
}
