use generational_arena::Index;

use crate::util::resource::Resource;

use super::{
    drawable::Drawable, physics::Physics, player::Player, projectile::projectile::Projectile,
};

pub type EntityHolder = Index;

pub struct Entity {
    pub resource: &'static Resource,
    pub physics: Option<Physics>,
    pub drawable: Option<Drawable<'static>>,
    pub player: Option<Player>,
    pub projectile: Option<Projectile>,
}
