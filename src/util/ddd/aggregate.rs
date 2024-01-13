use crate::util::ddd::Entity;

pub(crate) trait Aggregate: Entity {
    type Event;
}
