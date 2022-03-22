use macroquad::prelude::get_time;

use crate::{
    entity::{
        drawable::Drawable,
        entity::{Entity, EntityHolder},
        physics::{Physics, PhysicsLike},
    },
    world::world::World,
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
        drawable: &Drawable,
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
        drawable: &Drawable,
    ) -> Option<Self> {
        let pos = source_entity.pos()?;
        let rotation = source_entity.rotation()?;
        let velocity = 40.0 + source_entity.velocity()?;

        Some(Self {
            physics: Some(Physics {
                pos,
                velocity,
                rotation,
            }),
            drawable: Some(*drawable),
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
