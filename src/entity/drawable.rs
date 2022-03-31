use macroquad::prelude::{DrawTextureParams, Texture2D, WHITE};
use rapier2d::prelude::RigidBodySet;

use crate::util::{draw, screen::TWO};

use super::{entity::Entity, physics::PhysicsLike};

#[derive(Clone, Copy)]
pub struct Drawable {
    pub texture: Texture2D,
}

pub trait DrawableLike {
    fn draw(&self, rigid_body_set: &RigidBodySet) -> Option<()>;
}

impl DrawableLike for Entity {
    fn draw(&self, rigid_body_set: &RigidBodySet) -> Option<()> {
        let ref texture = self.drawable.as_ref()?.texture;

        let mut options = DrawTextureParams::default();
        options.rotation = self.rotation(rigid_body_set)?.angle();

        let pos = *self.pos(rigid_body_set)?;
        let (x, y) = (
            pos.x - texture.width() / TWO,
            pos.y - texture.height() / TWO,
        );

        draw::draw_texture_ex(texture, x, y, WHITE, options);

        Some(())
    }
}
