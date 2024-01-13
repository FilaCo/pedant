use crate::util::di::{Lifetime, Tag};
use crate::util::UtilResult;

pub(crate) struct Container {}

impl Container {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_singleton<T>(&mut self, tag: Option<Tag>) -> &mut Self {
        self.add::<T>(tag, Lifetime::Singleton)
    }

    pub fn add_scoped<T>(&mut self, tag: Option<Tag>) -> &mut Self {
        self.add::<T>(tag, Lifetime::Scoped)
    }

    pub fn add_transient<T>(&mut self, tag: Option<Tag>) -> &mut Self {
        self.add::<T>(tag, Lifetime::Transient)
    }

    pub fn add<T>(&mut self, tag: Option<Tag>, lifetime: Lifetime) -> &mut Self {
        self
    }

    pub fn resolve<T>(&self, tag: Option<&Tag>) -> UtilResult<&T> {
        todo!()
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
