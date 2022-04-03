use macroquad::prelude::get_time;
use nalgebra::vector;
use rapier2d::prelude::{ColliderSet, RigidBody, RigidBodySet};

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

fn set_projectile_physics(
    physics: &Physics,
    source_rigid_body: &RigidBody,
    rigid_body_set: &mut RigidBodySet,
) -> Option<()> {
    let rigid_body = rigid_body_set.get_mut(physics.rigid_body_handle)?;

    let rotation_v = source_rigid_body.rotation().scale(800.0);
    let velocity = source_rigid_body.linvel() + vector!(rotation_v.re, rotation_v.im);

    rigid_body.set_position(*source_rigid_body.position(), true);
    rigid_body.set_linvel(velocity, true);

    Some(())
}

impl ProjectileLike for Entity {
    fn spawn_projectile(
        source: EntityHolder,
        source_entity: &Entity,
        lifetime: f64,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Self> {
        let physics = Physics::from_resource(&BULLET, rigid_body_set, collider_set)?;
        let source_rigid_body = source_entity.get_rigid_body(rigid_body_set)?.clone();

        set_projectile_physics(&physics, &source_rigid_body, rigid_body_set);

        Some(Self {
            physics: Some(physics),
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
