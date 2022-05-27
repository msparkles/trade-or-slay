use rapier2d::prelude::{ColliderSet, RigidBodySet};

use crate::util::{draw, resource::Resource};

use super::{entity::Entity, physics::PhysicsLike};

pub struct Drawable<'a> {
    pub resource: &'a Resource,
}

impl Drawable<'_> {
    pub fn from_resource(resource: &Resource) -> Option<Drawable> {
        Some(Drawable { resource })
    }
}

pub trait DrawableLike {
    fn draw(&self, rigid_body_set: &RigidBodySet, collider_set: &ColliderSet) -> Option<()>;
}

impl DrawableLike for Entity {
    fn draw(&self, rigid_body_set: &RigidBodySet, collider_set: &ColliderSet) -> Option<()> {
        let resource = self.drawable.as_ref()?.resource;
        let transform = self.transform(rigid_body_set)?;

        draw::draw(resource, transform);

        if cfg!(debug_assertions) {
            let rigid_body = self.get_rigid_body(rigid_body_set)?;

            draw::draw_colliders(rigid_body, collider_set);
        }

        Some(())
    }
}
