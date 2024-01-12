use crate::infra::InfraError;

pub(crate) type InfraResult<T> = Result<T, InfraError>;
