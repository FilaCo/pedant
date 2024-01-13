use crate::util::ddd::VO;
use crate::util::UtilError;

pub(crate) struct Tag(String);

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl VO for Tag {
    type Error = UtilError;
}
