use crate::util::ddd::{Version, VO};

pub(crate) trait Entity: PartialEq {
    type Id: VO;
    type Event;

    fn id(&self) -> &Self::Id;

    fn version(&self) -> &Version;
}
