use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
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
