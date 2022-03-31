use macroquad::{
    camera::Camera2D,
    prelude::{Vec2, WHITE},
    texture,
};

use crate::{
    util::{
        draw,
        screen::{self, world_center, TWO},
    },
    CURSOR,
};

pub struct MouseInfo {
    pub pos: Vec2,
}

impl Default for MouseInfo {
    fn default() -> Self {
        Self {
            pos: world_center().into(),
        }
    }
}

impl MouseInfo {
    pub fn draw_cursor(&self) {
        let texture = &CURSOR.texture;
        let (x, y) = self.pos.into();
        let (x, y) = (x - texture.width() / TWO, y - texture.height() / TWO);

        texture::draw_texture(*texture, x, y, WHITE);
    }

    pub fn from_mouse(&mut self, camera: &Camera2D) {
        let (x, y) = screen::get_world_mouse_pos(camera).into();

        self.pos.x = x;
        self.pos.y = y;
    }
}
