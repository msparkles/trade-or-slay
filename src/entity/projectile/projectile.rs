use macroquad::prelude::get_time;
use nalgebra::vector;
use rapier2d::{math::Isometry, prelude::RigidBodySet};

use crate::{
    entity::{
        drawable::Drawable,
        entity::{Entity, EntityBuilder, EntityHolder},
        physics::PhysicsLike,
    },
    world::world_mutator::WorldMutator,
    BULLET,
};

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub source: EntityHolder,
    pub fired_time: f64,
    pub lifetime: f64,
}
pub trait ProjectileLike {
    fn spawn_projectile(
        source: EntityHolder,
        source_entity: Entity,
        lifetime: f64,
    ) -> Option<WorldMutator>;

    fn update_projectile(&self, current_time: f64) -> Option<WorldMutator>;
}

fn set_projectile_physics(
    entity: &mut Entity,
    source_entity: Entity,
    rigid_body_set: &mut RigidBodySet,
) -> Option<()> {
    let rotation = source_entity.rotation(&rigid_body_set)?;
    let rotation_v = rotation.scale(800.0);

    let velocity = source_entity.velocity(&rigid_body_set)? + vector!(rotation_v.re, rotation_v.im);

    let fire_point = source_entity.resource.info.fire_points()?.get_point2s()[0];
    let fire_point = source_entity
        .transform(&rigid_body_set)?
        .transform_point(&fire_point);

    let rigid_body = entity.get_rigid_body_mut(rigid_body_set)?;

    rigid_body.set_position(Isometry::new(fire_point.coords, rotation.angle()), false);
    rigid_body.set_linvel(velocity, false);

    Some(())
}

impl ProjectileLike for Entity {
    fn spawn_projectile(
        source: EntityHolder,
        source_entity: Entity,
        lifetime: f64,
    ) -> Option<WorldMutator> {
        let drawable = Drawable::from_resource(&BULLET)?;
        let projectile = Projectile {
            source,
            fired_time: get_time(),
            lifetime,
        };

        Some(
            EntityBuilder::new(&BULLET)
                .drawable(drawable)
                .projectile(projectile)
                .build_mutator(Box::new(move |entity, rigid_body_set| {
                    set_projectile_physics(entity, source_entity, rigid_body_set)?;
                    None
                })),
        )
    }

    fn update_projectile(&self, current_time: f64) -> Option<WorldMutator> {
        let time_elapsed = current_time - self.projectile.as_ref()?.fired_time;
        let lifetime = self.projectile.as_ref()?.lifetime;

        if time_elapsed > lifetime {
            return Some(WorldMutator::Remove(self.entity_holder?));
        }
        // todo add collision code
        None
    }
}
