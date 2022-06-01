use generational_arena::Index;

use crate::{
    util::resource::Resource,
    world::{
        world::World,
        world_mutator::{PostInitFn, WorldMutator},
    },
};

use super::{
    drawable::Drawable, physics::Physics, player::Player, projectile::projectile::Projectile,
};

pub type EntityHolder = Index;

#[derive(Clone, Copy)]
pub struct Entity {
    pub entity_holder: Option<EntityHolder>,
    pub resource: &'static Resource,
    pub physics: Option<Physics>,
    pub drawable: Option<Drawable<'static>>,
    pub player: Option<Player>,
    pub projectile: Option<Projectile>,
}

#[derive(Clone, Copy)]
pub struct EntityBuilder {
    entity: Entity,
}

impl EntityBuilder {
    pub fn new(resource: &'static Resource) -> Self {
        Self {
            entity: Entity {
                entity_holder: None,
                resource,
                physics: None,
                drawable: None,
                player: None,
                projectile: None,
            },
        }
    }

    pub fn force_build(self) -> Entity {
        self.entity
    }

    pub fn build_mutator(self, post_init: PostInitFn) -> WorldMutator {
        WorldMutator::Add(self.entity, post_init)
    }

    pub fn build_no_postinit(self) -> WorldMutator {
        WorldMutator::Add(self.entity, Box::new(|_, _| None))
    }

    pub fn build<'a>(self, world: &'a mut World, post_init: PostInitFn) -> Option<EntityHolder> {
        world.add_entity(self.entity, post_init)
    }

    pub fn drawable(&mut self, drawable: Drawable<'static>) -> &mut Self {
        self.entity.drawable = Some(drawable);

        return self;
    }

    pub fn player(&mut self, player: Player) -> &mut Self {
        self.entity.player = Some(player);

        return self;
    }

    pub fn projectile(&mut self, projectile: Projectile) -> &mut Self {
        self.entity.projectile = Some(projectile);

        return self;
    }
}
