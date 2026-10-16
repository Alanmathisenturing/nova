use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Error)]
#[error("{self:?}")]
pub enum CfqpError {
    InformationBoundaryViolation,
    SelfAuthorizationForbidden,
    InvalidTransition,
    InvalidVersion,
    MissingEvidence,
    InsufficientBasis,
    ContestNotAllowed,
    ReplayMismatch,
    CanonicalizationFailure,
    CorruptedEvaluation,
}
