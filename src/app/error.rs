use crate::domain::DomainError;
use crate::infra::InfraError;
use crate::util::UtilError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        match value {}
    }
}

impl From<InfraError> for AppError {
    fn from(value: InfraError) -> Self {
        match value {}
    }
}

impl From<UtilError> for AppError {
    fn from(value: UtilError) -> Self {
        match value {}
    }
}
