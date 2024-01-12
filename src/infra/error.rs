use crate::domain::DomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum InfraError {}

impl From<DomainError> for InfraError {
    fn from(value: DomainError) -> Self {
        match value {}
    }
}
