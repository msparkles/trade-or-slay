use macroquad::{camera::Camera2D, prelude::Vec2};
use rapier2d::math::Isometry;

use crate::{
    util::{
        draw,
        screen::{self, world_center, TWO},
    },
    CURSOR,
};

#[derive(Debug)]
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
        let cursor = &CURSOR;
        let (x, y) = self.pos.into();
        let (x, y) = (x - cursor.width / TWO, y - cursor.height / TWO);

        draw::draw(cursor, Isometry::translation(x, y));
    }

    pub fn from_mouse(&mut self, camera: &Camera2D) {
        let (x, y) = screen::get_world_mouse_pos(camera).into();

        self.pos.x = x;
        self.pos.y = y;
    }
}
