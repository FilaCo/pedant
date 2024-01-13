use crate::util::ddd::VO;

pub(crate) trait Entity: PartialEq {
    type Id: VO;

    fn id(&self) -> &Self::Id;
}
