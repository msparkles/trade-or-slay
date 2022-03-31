use std::{borrow::BorrowMut, cell::RefCell};

use generational_arena::{Arena, Index};
use macroquad::{
    camera::Camera2D,
    prelude::{get_time, vec2},
};
use rapier2d::prelude::{ColliderSet, RigidBodySet};

use crate::{
    entity::{
        drawable::DrawableLike,
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
    pub rigid_body_set: RefCell<RigidBodySet>,
    pub collider_set: RefCell<ColliderSet>,
}

impl World {
    pub fn get_entity_mut(&mut self, holder: &EntityHolder) -> Option<&mut Entity> {
        self.entities.get_mut(*holder)
    }

    pub fn get_entity(&self, holder: &EntityHolder) -> Option<&Entity> {
        self.entities.get(*holder)
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityHolder {
        self.entities.insert(entity)
    }

    pub fn set_player(&mut self, player: Entity) {
        self.player = Some(self.add_entity(player));
    }

    fn mouse(&mut self, player: &Index, camera: &mut Camera2D) -> Option<()> {
        let player_entity = self.get_entity_mut(&player)?;

        let mouse_info = player_entity.mouse_info_mut()?;
        mouse_info.from_mouse(&camera);
        mouse_info.draw_cursor();

        Some(())
    }

    pub fn update(&mut self, camera: &mut Camera2D) -> Option<()> {
        draw_bg();

        let projectile = {
            let player = self.player?;

            {
                let mut rigid_body_set = self.rigid_body_set.borrow_mut();

                let player_entity = self.get_entity(&player)?;
                let pos = *player_entity.pos(rigid_body_set.borrow_mut())?;
                camera.target = vec2(pos.x, pos.y);
            }

            self.mouse(&player, camera);

            let mut rigid_body_set = self.rigid_body_set.borrow_mut();
            let mut collider_set = self.collider_set.borrow_mut();

            let player_entity = self.get_entity(&player)?;
            player_entity.update_input(rigid_body_set.borrow_mut())?;

            player_entity.update_fire(
                &player,
                rigid_body_set.borrow_mut(),
                collider_set.borrow_mut(),
            )
        };

        if let Some(projectile) = projectile {
            self.add_entity(projectile);
        }

        let mut rigid_body_set = self.rigid_body_set.borrow_mut();
        let mut collider_set = self.collider_set.borrow_mut();

        let current_time = get_time();
        let mut to_remove: Vec<EntityHolder> = vec![];

        for (index, entity) in self.entities.iter_mut() {
            entity.update_entity_position(rigid_body_set.borrow_mut());

            if entity.update_projectile(current_time).unwrap_or(false) {
                to_remove.push(index);
            }

            entity.draw(rigid_body_set.borrow_mut());
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
            rigid_body_set: RefCell::new(RigidBodySet::new()),
            collider_set: RefCell::new(ColliderSet::new()),
        }
    }
}
