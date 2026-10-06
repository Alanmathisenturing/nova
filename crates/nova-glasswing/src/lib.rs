use nova_types::{Digest, EventId, StateRoot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub name: String,
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority {
    pub actor: String,
    pub capability: Capability,
    pub expires_at: Option<u64>,
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Action {
    pub actor: String,
    pub resource: String,
    pub capability: String,
    pub event: EventId,
    pub state_root: StateRoot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permit {
    pub action_digest: Digest,
    pub capability: String,
    pub actor: String,
    pub state_root: StateRoot,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PolicyError {
    Denied,
    ScopeMismatch,
    Revoked,
    Expired,
    StateRootMismatch,
}

impl Action {
    pub fn digest(&self) -> Digest {
        let mut b = Vec::new();
        b.extend_from_slice(self.actor.as_bytes());
        b.extend_from_slice(&(self.actor.len() as u64).to_le_bytes());
        b.extend_from_slice(self.resource.as_bytes());
        b.extend_from_slice(&(self.resource.len() as u64).to_le_bytes());
        b.extend_from_slice(self.capability.as_bytes());
        b.extend_from_slice(&(self.capability.len() as u64).to_le_bytes());
        b.extend_from_slice(&self.event.0.to_le_bytes());
        b.extend_from_slice(&self.state_root.0);
        Digest::of(b"nova.action.v2", &b)
    }
}

pub fn issue(
    authority: &Authority,
    action: &Action,
    now: u64,
) -> Result<Permit, PolicyError> {
    if authority.revoked {
        return Err(PolicyError::Revoked);
    }
    if authority.actor != action.actor || authority.capability.name != action.capability {
        return Err(PolicyError::Denied);
    }
    if authority.capability.scope != "*" && authority.capability.scope != action.resource {
        return Err(PolicyError::ScopeMismatch);
    }
    if authority.expires_at.is_some_and(|expiry| now > expiry) {
        return Err(PolicyError::Expired);
    }

    Ok(Permit {
        action_digest: action.digest(),
        capability: authority.capability.name.clone(),
        actor: authority.actor.clone(),
        state_root: action.state_root,
        expires_at: authority.expires_at,
    })
}

pub fn authorize(
    permit: &Permit,
    action: &Action,
    now: u64,
) -> Result<(), PolicyError> {
    if permit.action_digest != action.digest()
        || permit.actor != action.actor
        || permit.capability != action.capability
    {
        return Err(PolicyError::Denied);
    }
    if permit.state_root != action.state_root {
        return Err(PolicyError::StateRootMismatch);
    }
    if permit.expires_at.is_some_and(|expiry| now > expiry) {
        return Err(PolicyError::Expired);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(n: u8) -> StateRoot {
        StateRoot([n; 32])
    }

    fn authority() -> Authority {
        Authority {
            actor: "agent-1".into(),
            capability: Capability {
                name: "write".into(),
                scope: "vault".into(),
            },
            expires_at: Some(100),
            revoked: false,
        }
    }

    fn action(root: StateRoot) -> Action {
        Action {
            actor: "agent-1".into(),
            resource: "vault".into(),
            capability: "write".into(),
            event: EventId(1),
            state_root: root,
        }
    }

    #[test]
    fn issue_binds_authority_to_action_and_state_root() {
        let a = action(root(1));
        let p = issue(&authority(), &a, 50).unwrap();
        assert_eq!(p.action_digest, a.digest());
        assert_eq!(p.state_root, root(1));
    }

    #[test]
    fn revoked_authority_cannot_issue_permit() {
        let mut a = authority();
        a.revoked = true;
        assert_eq!(issue(&a, &action(root(1)), 50), Err(PolicyError::Revoked));
    }

    #[test]
    fn expired_authority_cannot_issue_permit() {
        assert_eq!(issue(&authority(), &action(root(1)), 101), Err(PolicyError::Expired));
    }

    #[test]
    fn scope_escalation_is_rejected() {
        let mut a = authority();
        a.capability.scope = "other".into();
        assert_eq!(issue(&a, &action(root(1)), 50), Err(PolicyError::ScopeMismatch));
    }

    #[test]
    fn permit_rejects_stale_state_root() {
        let a = action(root(1));
        let p = issue(&authority(), &a, 50).unwrap();
        let stale = action(root(2));
        assert_eq!(authorize(&p, &stale, 50), Err(PolicyError::Denied));
    }

    #[test]
    fn permit_expiry_is_enforced_at_authorization() {
        let a = action(root(1));
        let p = issue(&authority(), &a, 50).unwrap();
        assert_eq!(authorize(&p, &a, 101), Err(PolicyError::Expired));
    }

    #[test]
    fn identical_actions_have_identical_digests() {
        assert_eq!(action(root(1)).digest(), action(root(1)).digest());
    }
}
