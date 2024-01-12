use crate::app::result::AppResult;

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> AppResult<()> {
        Ok(())
    }
}
