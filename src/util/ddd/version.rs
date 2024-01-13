use crate::util::ddd::VO;
use crate::util::UtilError;

pub(crate) struct Version(u64);

impl Version {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl VO for Version {
    type Error = UtilError;
}
