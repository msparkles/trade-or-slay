use std::borrow::Borrow;

use generational_arena::Arena;
use macroquad::{camera::Camera2D, prelude::get_time};

use crate::{
    entity::{
        drawable::{Drawable, DrawableLike},
        entity::{Entity, EntityHolder},
        physics::PhysicsLike,
        player::PlayerLike,
        projectile::projectile::ProjectileLike,
    },
    util::bg::draw_bg,
};
pub struct World {
    pub entities: Arena<Entity>,
    pub player: Option<EntityHolder>,
}

impl World {
    pub fn get_entity(&mut self, holder: &EntityHolder) -> Option<&mut Entity> {
        self.entities.get_mut(*holder)
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityHolder {
        self.entities.insert(entity)
    }

    pub fn set_player(&mut self, player: Entity) {
        self.player = Some(self.add_entity(player));
    }

    pub fn update(&mut self, camera: &mut Camera2D, drawable: &Drawable) -> Option<()> {
        let player = self.player?;

        let player_entity = self.get_entity(&player)?;

        //let area = player_entity.area_of_map()?;

        draw_bg();

        player_entity.update_input()?;
        let projectile = player_entity.update_fire(&player, drawable);

        camera.target = player_entity.pos()?;

        if let Some(projectile) = projectile {
            self.add_entity(projectile);
        }

        let current_time = get_time();
        let mut to_remove: Vec<EntityHolder> = vec![];

        for (index, entity) in self.entities.iter_mut() {
            entity.update_entity_position()?;

            if entity.update_projectile(current_time).unwrap_or(false) {
                to_remove.push(index);
            }

            entity.draw()?;
        }

        for index in to_remove {
            self.entities.remove(index);
        }

        Some(())
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            entities: Arena::new(),
            player: None,
        }
    }
}
