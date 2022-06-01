use std::cell::RefCell;

use generational_arena::{Arena, Index};
use macroquad::{
    camera::Camera2D,
    prelude::{get_time, vec2},
};
use rapier2d::{
    crossbeam::channel::Receiver,
    math::Isometry,
    prelude::{
        ColliderSet, ContactEvent, IntersectionEvent, IslandManager, JointSet, RigidBodySet,
    },
};

use crate::{
    entity::{
        drawable::{Drawable, DrawableLike},
        entity::{Entity, EntityBuilder, EntityHolder},
        physics::{Physics, PhysicsLike},
        player::PlayerLike,
        projectile::projectile::ProjectileLike,
    },
    util::{bg::draw_bg, math::random_place_on_map},
    SHIP,
};

use super::world_mutator::{PostInitFn, WorldMutator};
pub struct World {
    pub entities: Arena<Entity>,
    pub player: Option<EntityHolder>,
    pub rigid_body_set: RefCell<RigidBodySet>,
    pub collider_set: RefCell<ColliderSet>,
    pub island_manager: RefCell<IslandManager>,
    pub joint_set: RefCell<JointSet>,
}

fn add_entity_property(
    entity: &mut Entity,
    entity_holder: EntityHolder,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) -> Option<()> {
    let rigid_body = entity.resource.info.rigid_body.as_ref()?.clone();
    let collider = entity.resource.info.collider.as_ref()?.clone();

    let rigid_body_handle = rigid_body_set.insert(rigid_body);
    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    entity.physics = Some(Physics {
        rigid_body_handle,
        collider_handle,
    });

    entity.entity_holder = Some(entity_holder);

    Some(())
}

impl World {
    fn handle_mutator(&mut self, world_mutator: WorldMutator) -> Option<EntityHolder> {
        match world_mutator {
            WorldMutator::Remove(entity_holder) => {
                self.remove_entity(entity_holder);
                None
            }
            WorldMutator::Add(entity, post_init) => self.add_entity(entity, post_init),
        }
    }

    pub fn get_entity_mut(&mut self, holder: &EntityHolder) -> Option<&mut Entity> {
        self.entities.get_mut(*holder)
    }

    pub fn get_entity(&self, holder: &EntityHolder) -> Option<&Entity> {
        self.entities.get(*holder)
    }

    pub fn remove_entity(&mut self, entity_holder: EntityHolder) -> Entity {
        let entity = self
            .entities
            .remove(entity_holder)
            .expect("entity was never in the arena");

        if let Some(handle) = entity.physics.and_then(|v| Some(v.rigid_body_handle)) {
            let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();
            let collider_set = &mut *self.collider_set.borrow_mut();
            let island_manager = &mut *self.island_manager.borrow_mut();
            let joint_set = &mut *self.joint_set.borrow_mut();

            rigid_body_set.remove(handle, island_manager, collider_set, joint_set);
        }

        return entity;
    }

    pub fn add_entity(&mut self, entity: Entity, post_init: PostInitFn) -> Option<EntityHolder> {
        let entity_holder = self.entities.insert(entity);

        let entity = self
            .entities
            .get_mut(entity_holder)
            .expect("somehow entity isn't present right after insertion");

        {
            let result = {
                let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();
                let collider_set = &mut *self.collider_set.borrow_mut();

                add_entity_property(entity, entity_holder, rigid_body_set, collider_set)
            };

            if let None = result {
                self.remove_entity(entity_holder);

                return None;
            }
        }

        {
            let result = {
                let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();

                post_init(entity, rigid_body_set)
            };

            result.and_then(|v| {
                self.handle_mutator(v);
                Some(())
            });
        }

        Some(entity_holder)
    }

    pub fn set_player(&mut self, world_mutator: WorldMutator) -> Option<()> {
        self.player = self.handle_mutator(world_mutator);

        Some(())
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

        let rigid_body_set = &mut self.rigid_body_set.borrow();

        let pos = *player_entity.pos(rigid_body_set)?;
        camera.target = vec2(pos.x, pos.y);

        Some(())
    }

    fn input(&mut self, player: &Index) -> Option<()> {
        let player_entity = self.get_entity(&player)?;
        let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();

        let rigid_body = player_entity.get_rigid_body_mut(rigid_body_set)?;

        player_entity.update_input(rigid_body);

        Some(())
    }

    fn fire(&mut self, player: &Index) -> Option<WorldMutator> {
        let player_entity = self.get_entity_mut(player)?;

        player_entity.update_fire(&player)
    }

    fn spawn_enemy(&mut self) -> Option<Vec<WorldMutator>> {
        let p = random_place_on_map();

        let drawable = Drawable::from_resource(&SHIP)?;

        Some(vec![EntityBuilder::new(&SHIP)
            .drawable(drawable)
            .build_mutator(Box::new(move |entity, rigid_body_set| {
                entity
                    .get_rigid_body_mut(rigid_body_set)?
                    .set_position(Isometry::translation(p.0, p.1), false);

                None
            }))])
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

        self.fire(&player).and_then(|v| {
            self.handle_mutator(v);
            Some(())
        });

        if current_time % 1.0 <= 0.1 {
            self.spawn_enemy().and_then(|v| {
                v.into_iter().for_each(|e| {
                    self.handle_mutator(e);
                });
                Some(())
            });
        }

        let mut to_remove: Vec<EntityHolder> = vec![];
        {
            let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();

            for (_, entity) in self.entities.iter_mut() {
                entity.update_entity_position(rigid_body_set);

                if let Some(WorldMutator::Remove(entity_holder)) =
                    entity.update_projectile(current_time)
                {
                    to_remove.push(entity_holder);
                }
            }
        }

        while let Ok(intersection_event) = intersection_recv.try_recv() {
            log::debug!("Received intersection event: {:?}", intersection_event);
        }

        /*
        while let Ok(contact_event) = contact_recv.try_recv() {
            log::debug!("Received contact event: {:?}", contact_event);
        }
        */

        to_remove.into_iter().for_each(|index| {
            self.remove_entity(index);
        });

        let rigid_body_set = &mut *self.rigid_body_set.borrow_mut();
        let collider_set = &*self.collider_set.borrow();

        self.entities.iter_mut().for_each(|(_, entity)| {
            entity.draw(&rigid_body_set, &collider_set);
        });

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
            island_manager: RefCell::new(IslandManager::new()),
            joint_set: RefCell::new(JointSet::new()),
        }
    }
}
