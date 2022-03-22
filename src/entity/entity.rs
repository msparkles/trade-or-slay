use generational_arena::Index;

use super::{
    drawable::Drawable, physics::Physics, player::Player, projectile::projectile::Projectile,
};

pub type EntityHolder = Index;

pub struct Entity {
    pub physics: Option<Physics>,
    pub drawable: Option<Drawable>,
    pub player: Option<Player>,
    pub projectile: Option<Projectile>,
}
