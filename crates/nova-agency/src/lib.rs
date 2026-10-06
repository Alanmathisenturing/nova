use nova_adjudication::{AgencyId, AdjudicationRoot, VerdictClass};
use nova_types::{Digest, StateRoot};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgencyStatus {
    Active,
    Probation,
    Suspended,
    Revoked,
}

impl AgencyStatus {
    fn tag(self) -> u8 {
        match self {
            Self::Active => 0,
            Self::Probation => 1,
            Self::Suspended => 2,
            Self::Revoked => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgencyTransitionReason {
    QualificationSupported,
    QualificationWeaklySupported,
    QualificationContradicted,
    QualificationInvalid,
    QualificationRejected,
    QualificationUnderdetermined,
}

impl AgencyTransitionReason {
    fn tag(self) -> u8 {
        match self {
            Self::QualificationSupported => 0,
            Self::QualificationWeaklySupported => 1,
            Self::QualificationContradicted => 2,
            Self::QualificationInvalid => 3,
            Self::QualificationRejected => 4,
            Self::QualificationUnderdetermined => 5,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgencyState {
    pub id: AgencyId,
    pub epoch: u64,
    pub status: AgencyStatus,
    pub qualification_root: Digest,
    pub adjudication_root: Option<AdjudicationRoot>,
    pub prior_state_root: StateRoot,
}

impl AgencyState {
    pub fn root(&self) -> StateRoot {
        let mut b = Vec::new();
        b.extend_from_slice(&self.id.0.to_le_bytes());
        b.extend_from_slice(&self.epoch.to_le_bytes());
        b.push(self.status.tag());
        b.extend_from_slice(&self.qualification_root.0);
        match self.adjudication_root {
            Some(root) => {
                b.push(1);
                b.extend_from_slice(&root.0.0);
            }
            None => b.push(0),
        }
        b.extend_from_slice(&self.prior_state_root.0);
        StateRoot(Digest::of(b"nova.agency_state_root.v1", &b).0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgencyTransition {
    pub agency_id: AgencyId,
    pub prior_root: StateRoot,
    pub next_root: StateRoot,
    pub reason: AgencyTransitionReason,
    pub adjudication_root: AdjudicationRoot,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AgencyTransitionError {
    AgencyMismatch,
    StaleStateRoot,
}

pub fn transition(
    current: &AgencyState,
    expected_prior_root: StateRoot,
    target_agency_id: AgencyId,
    adjudication_root: AdjudicationRoot,
    verdict: VerdictClass,
) -> Result<(AgencyState, AgencyTransition), AgencyTransitionError> {
    if current.id != target_agency_id {
        return Err(AgencyTransitionError::AgencyMismatch);
    }
    if current.root() != expected_prior_root {
        return Err(AgencyTransitionError::StaleStateRoot);
    }
    let next_status = match verdict {
        VerdictClass::Supported => AgencyStatus::Active,
        VerdictClass::WeaklySupported => AgencyStatus::Probation,
        VerdictClass::Contradicted => AgencyStatus::Suspended,
        VerdictClass::Invalid | VerdictClass::Rejected => AgencyStatus::Revoked,
        VerdictClass::Underdetermined => current.status,
    };

    let reason = match verdict {
        VerdictClass::Supported => AgencyTransitionReason::QualificationSupported,
        VerdictClass::WeaklySupported => AgencyTransitionReason::QualificationWeaklySupported,
        VerdictClass::Contradicted => AgencyTransitionReason::QualificationContradicted,
        VerdictClass::Invalid => AgencyTransitionReason::QualificationInvalid,
        VerdictClass::Rejected => AgencyTransitionReason::QualificationRejected,
        VerdictClass::Underdetermined => AgencyTransitionReason::QualificationUnderdetermined,
    };

    let next = AgencyState {
        id: current.id,
        epoch: current.epoch + 1,
        status: next_status,
        qualification_root: current.qualification_root,
        adjudication_root: Some(adjudication_root),
        prior_state_root: current.root(),
    };
    let prior_root = current.root();
    let next_root = next.root();
    Ok((next, AgencyTransition {
        agency_id: current.id,
        prior_root,
        next_root,
        reason,
        adjudication_root,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agency() -> AgencyState {
        AgencyState {
            id: AgencyId(7),
            epoch: 3,
            status: AgencyStatus::Probation,
            qualification_root: Digest::of(b"qualification", b"q1"),
            adjudication_root: None,
            prior_state_root: StateRoot([0; 32]),
        }
    }

    fn adjudication_root() -> AdjudicationRoot {
        AdjudicationRoot(Digest::of(b"adjudication", b"a1"))
    }

    #[test]
    fn supported_verdict_promotes_agency() {
        let (next, tx) = transition(&agency(), agency().root(), AgencyId(7), adjudication_root(), VerdictClass::Supported).unwrap();
        assert_eq!(next.status, AgencyStatus::Active);
        assert_eq!(next.epoch, 4);
        assert_eq!(tx.prior_root, agency().root());
        assert_eq!(tx.next_root, next.root());
    }

    #[test]
    fn contradicted_verdict_suspends_agency() {
        let (next, _) = transition(&agency(), agency().root(), AgencyId(7), adjudication_root(), VerdictClass::Contradicted).unwrap();
        assert_eq!(next.status, AgencyStatus::Suspended);
    }

    #[test]
    fn rejected_verdict_revokes_agency() {
        let (next, _) = transition(&agency(), agency().root(), AgencyId(7), adjudication_root(), VerdictClass::Rejected).unwrap();
        assert_eq!(next.status, AgencyStatus::Revoked);
    }

    #[test]
    fn underdetermined_verdict_preserves_status_but_advances_history() {
        let current = agency();
        let (next, _) = transition(&current, current.root(), AgencyId(7), adjudication_root(), VerdictClass::Underdetermined).unwrap();
        assert_eq!(next.status, current.status);
        assert_eq!(next.epoch, current.epoch + 1);
        assert_ne!(next.root(), current.root());
    }


    #[test]
    fn stale_root_is_rejected() {
        let current = agency();
        let err = transition(
            &current,
            StateRoot([9; 32]),
            AgencyId(7),
            adjudication_root(),
            VerdictClass::Supported,
        ).unwrap_err();
        assert_eq!(err, AgencyTransitionError::StaleStateRoot);
    }

    #[test]
    fn wrong_agency_is_rejected() {
        let current = agency();
        let err = transition(
            &current,
            current.root(),
            AgencyId(8),
            adjudication_root(),
            VerdictClass::Supported,
        ).unwrap_err();
        assert_eq!(err, AgencyTransitionError::AgencyMismatch);
    }

    #[test]
    fn different_adjudication_changes_next_root() {
        let current = agency();
        let (a, _) = transition(&current, current.root(), AgencyId(7), adjudication_root(), VerdictClass::Supported).unwrap();
        let (b, _) = transition(
            &current,
            current.root(),
            AgencyId(7),
            AdjudicationRoot(Digest::of(b"adjudication", b"a2")),
            VerdictClass::Supported,
        ).unwrap();
        assert_ne!(a.root(), b.root());
    }
}
