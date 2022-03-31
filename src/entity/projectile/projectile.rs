use macroquad::prelude::get_time;
use nalgebra::vector;
use rapier2d::prelude::{ColliderSet, RigidBodySet};

use crate::{
    entity::{
        drawable::Drawable,
        entity::{Entity, EntityHolder},
        physics::{Physics, PhysicsLike},
    },
    BULLET,
};

pub struct Projectile {
    pub source: EntityHolder,
    pub fired_time: f64,
    pub lifetime: f64,
}
pub trait ProjectileLike {
    fn spawn_projectile(
        source: EntityHolder,
        source_entity: &Entity,
        lifetime: f64,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Self>
    where
        Self: Sized;

    fn update_projectile(&self, current_time: f64) -> Option<bool>;
}

impl ProjectileLike for Entity {
    fn spawn_projectile(
        source: EntityHolder,
        source_entity: &Entity,
        lifetime: f64,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Self> {
        let mut rigid_body = BULLET.rigid_body.clone()?;
        let collider = BULLET.collider.clone()?;

        let original = source_entity.get_rigid_body(rigid_body_set)?;
        let rotation_v = original.rotation().scale(800.0);
        let velocity = original.linvel() + vector!(rotation_v.re, rotation_v.im);

        rigid_body.set_position(*original.position(), true);
        rigid_body.set_linvel(velocity, true);

        Some(Self {
            physics: Some(Physics {
                rigid_body: rigid_body_set.insert(rigid_body),
                collider: collider_set.insert(collider),
            }),
            drawable: Some(Drawable {
                texture: BULLET.texture.clone(),
            }),
            player: None,
            projectile: Some(Projectile {
                source,
                fired_time: get_time(),
                lifetime,
            }),
        })
    }

    fn update_projectile(&self, current_time: f64) -> Option<bool> {
        let time_elapsed = current_time - self.projectile.as_ref()?.fired_time;
        let lifetime = self.projectile.as_ref()?.lifetime;

        if time_elapsed > lifetime {
            return Some(true);
        }
        // todo add collision code
        Some(false)
    }
}
