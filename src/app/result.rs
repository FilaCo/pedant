use crate::app::AppError;

pub type AppResult<T> = Result<T, AppError>;
