use std::{borrow::Borrow, cell::RefCell};

use generational_arena::{Arena, Index};
use macroquad::{
    camera::Camera2D,
    prelude::{get_time, vec2},
};
use rapier2d::{
    crossbeam::channel::Receiver,
    math::Isometry,
    prelude::{ColliderSet, ContactEvent, IntersectionEvent, RigidBodySet},
};

use crate::{
    entity::{
        drawable::{Drawable, DrawableLike},
        entity::{Entity, EntityHolder},
        physics::{Physics, PhysicsLike},
        player::PlayerLike,
        projectile::projectile::ProjectileLike,
    },
    util::{bg::draw_bg, math::random_place_on_map},
    SHIP,
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

    pub fn add_entities(&mut self, entities: Vec<Entity>) -> Vec<EntityHolder> {
        entities
            .into_iter()
            .map(|e| self.entities.insert(e))
            .collect()
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

    fn camera(&self, player: &Index, camera: &mut Camera2D) -> Option<()> {
        let player_entity = self.get_entity(&player)?;

        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        let pos = *player_entity.pos(&mut rigid_body_set)?;
        camera.target = vec2(pos.x, pos.y);

        Some(())
    }

    fn input(&self, player: &Index) -> Option<()> {
        let player_entity = self.get_entity(&player)?;

        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        player_entity.update_input(&mut rigid_body_set)?;

        Some(())
    }

    fn fire(&self, player: &Index) -> Option<Entity> {
        let player_entity = self.get_entity(&player)?;

        let mut rigid_body_set = self.rigid_body_set.borrow_mut();
        let mut collider_set = self.collider_set.borrow_mut();

        player_entity.update_fire(&player, &mut rigid_body_set, &mut collider_set)
    }

    fn spawn_enemy(&mut self) -> Option<Vec<Entity>> {
        let mut collider_set = self.collider_set.borrow_mut();
        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        let p = random_place_on_map();
        let physics = Physics::from_resource(&SHIP, &mut rigid_body_set, &mut collider_set)?;
        let rigid_body = rigid_body_set.get_mut(physics.rigid_body_handle)?;
        rigid_body.set_position(Isometry::translation(p.0, p.1), false);

        Some(vec![Entity {
            physics: Some(physics),
            drawable: Drawable::from_resource(&SHIP),
            player: None,
            projectile: None,
        }])
    }

    pub fn update(
        &mut self,
        contact_recv: &Receiver<ContactEvent>,
        intersection_recv: &Receiver<IntersectionEvent>,
        camera: &mut Camera2D,
    ) -> Option<()> {
        let current_time = get_time();

        draw_bg();

        let player = self.player?;

        self.camera(&player, camera);
        self.mouse(&player, camera);
        self.input(&player);

        let projectile = self.fire(&player);

        if let Some(projectile) = projectile {
            self.add_entity(projectile);
        }

        if current_time % 1.0 <= 0.1 {
            if let Some(enemies) = self.spawn_enemy() {
                self.add_entities(enemies);
            }
        }

        let mut rigid_body_set = self.rigid_body_set.borrow_mut();
        let collider_set = self.collider_set.borrow_mut();

        let mut to_remove: Vec<EntityHolder> = vec![];

        while let Ok(intersection_event) = intersection_recv.try_recv() {
            // Handle the intersection event.
            println!("Received intersection event: {:?}", intersection_event);
        }

        while let Ok(contact_event) = contact_recv.try_recv() {
            // Handle the contact event.
            println!("Received contact event: {:?}", contact_event);
        }

        for (index, entity) in self.entities.iter_mut() {
            entity.update_entity_position(&mut rigid_body_set);

            if entity.update_projectile(current_time).unwrap_or(false) {
                to_remove.push(index);
            }

            entity.draw(rigid_body_set.borrow(), collider_set.borrow());
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
