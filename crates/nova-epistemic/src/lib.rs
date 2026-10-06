use nova_types::Digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub source: String,
    pub observation: Vec<u8>,
    pub digest: Digest,
}

impl Evidence {
    pub fn new(source: impl Into<String>, observation: Vec<u8>) -> Self {
        let source = source.into();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(source.len() as u64).to_le_bytes());
        bytes.extend_from_slice(source.as_bytes());
        bytes.extend_from_slice(&(observation.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&observation);
        let digest = Digest::of(b"nova.epistemic.evidence.v2", &bytes);
        Self { source, observation, digest }
    }

    pub fn validate(&self) -> Result<(), EpistemicError> {
        if self.source.is_empty() || self.observation.is_empty() {
            return Err(EpistemicError::MissingEvidence);
        }
        if self.digest != Self::new(self.source.clone(), self.observation.clone()).digest {
            return Err(EpistemicError::EvidenceIntegrityViolation);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BeliefStatus {
    Unknown,
    Hypothetical,
    Plausible,
    Supported,
    StronglySupported,
    Contradicted,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Belief {
    pub statement: String,
    pub status: BeliefStatus,
    pub confidence: u8,
    pub evidence: Vec<Digest>,
}

impl Belief {
    pub fn validate(&self) -> Result<(), EpistemicError> {
        if self.statement.is_empty() || self.confidence > 100 {
            return Err(EpistemicError::InvalidBelief);
        }
        if self.status != BeliefStatus::Unknown && self.evidence.is_empty() {
            return Err(EpistemicError::MissingEvidence);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Prediction {
    pub belief: Digest,
    pub expected_outcome: Digest,
    pub horizon: u64,
}

impl Prediction {
    pub fn digest(&self) -> Digest {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.belief.0);
        bytes.extend_from_slice(&self.expected_outcome.0);
        bytes.extend_from_slice(&self.horizon.to_le_bytes());
        Digest::of(b"nova.epistemic.prediction.v1", &bytes)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EpistemicError {
    MissingEvidence,
    EvidenceIntegrityViolation,
    InvalidBelief,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_digest_binds_source_and_content() {
        let a = Evidence::new("source-a", b"observation".to_vec());
        let b = Evidence::new("source-b", b"observation".to_vec());
        assert_ne!(a.digest, b.digest);
        assert!(a.validate().is_ok());
    }

    #[test]
    fn tampered_epistemic_evidence_is_rejected() {
        let mut e = Evidence::new("source", b"observation".to_vec());
        e.observation[0] ^= 1;
        assert_eq!(e.validate(), Err(EpistemicError::EvidenceIntegrityViolation));
    }

    #[test]
    fn supported_belief_requires_evidence() {
        let belief = Belief { statement: "x".into(), status: BeliefStatus::Supported, confidence: 80, evidence: vec![] };
        assert_eq!(belief.validate(), Err(EpistemicError::MissingEvidence));
    }

    #[test]
    fn unknown_belief_can_have_no_evidence() {
        let belief = Belief { statement: "x".into(), status: BeliefStatus::Unknown, confidence: 0, evidence: vec![] };
        assert!(belief.validate().is_ok());
    }

    #[test]
    fn confidence_above_100_is_rejected() {
        let belief = Belief { statement: "x".into(), status: BeliefStatus::Unknown, confidence: 101, evidence: vec![] };
        assert_eq!(belief.validate(), Err(EpistemicError::InvalidBelief));
    }
}
