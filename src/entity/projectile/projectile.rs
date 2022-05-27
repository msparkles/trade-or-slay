use macroquad::prelude::get_time;
use nalgebra::vector;
use rapier2d::{
    math::Isometry,
    prelude::{ColliderSet, RigidBodySet},
};

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
    source_entity: &Entity,
    physics: &Physics,
    rigid_body_set: &mut RigidBodySet,
) -> Option<()> {
    let rotation = source_entity.rotation(rigid_body_set)?;

    let rotation_v = rotation.scale(800.0);
    let velocity = source_entity.velocity(rigid_body_set)? + vector!(rotation_v.re, rotation_v.im);

    let position = source_entity.transform(rigid_body_set)?;
    let fire_point = source_entity.resource.info.fire_points()?.to_points()[0];
    let fire_point = position.transform_point(&fire_point);

    let rigid_body = rigid_body_set.get_mut(physics.rigid_body_handle)?;

    rigid_body.set_position(Isometry::new(fire_point.coords, rotation.angle()), false);
    rigid_body.set_linvel(velocity, false);

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

        set_projectile_physics(&source_entity, &physics, rigid_body_set);

        Some(Self {
            resource: &BULLET,
            physics: Some(physics),
            drawable: Drawable::from_resource(&BULLET),
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
