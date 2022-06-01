use rapier2d::prelude::RigidBodySet;

use crate::entity::entity::{Entity, EntityHolder};

pub type PostInitFn =
    Box<dyn FnOnce(&mut Entity, &mut RigidBodySet) -> Option<WorldMutator> + Send + Sync>;

#[must_use]
pub enum WorldMutator {
    Remove(EntityHolder),
    Add(Entity, PostInitFn),
}
