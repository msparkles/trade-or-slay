use macroquad::prelude::{DrawTextureParams, Texture2D, WHITE};

use crate::util::{draw, screen::TWO};

use super::{entity::Entity, physics::PhysicsLike, player::MapArea};

#[derive(Clone, Copy)]
pub struct Drawable {
    pub texture: Texture2D,
}

pub trait DrawableLike {
    fn draw(&self) -> Option<()>;
}

impl DrawableLike for Entity {
    fn draw(&self) -> Option<()> {
        let texture = self.drawable.as_ref()?.texture;

        let mut options = DrawTextureParams::default();
        options.rotation = self.rotation()?;

        let (x, y) = self.pos()?.into();

        draw::draw_texture_ex(
            &texture,
            x - texture.width() / TWO,
            y - texture.height() / TWO,
            WHITE,
            options,
        );

        Some(())
    }
}
