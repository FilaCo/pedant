use crate::app::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
