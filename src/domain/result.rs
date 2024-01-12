use crate::domain::DomainError;

pub(crate) type DomainResult<T> = Result<T, DomainError>;
