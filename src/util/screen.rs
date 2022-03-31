use macroquad::{
    camera::Camera2D,
    prelude::{mouse_position, vec2, Vec2},
    window::*,
};
use nalgebra::{coordinates::XY, point};
use rapier2d::math::{Point, Real};

pub const TWO: f32 = 2.0;
pub const THREE: f32 = 3.0;

pub fn screen_center() -> (f32, f32) {
    (screen_width() / TWO, screen_height() / TWO)
}

pub fn screen_size() -> (f32, f32) {
    (screen_width(), screen_height())
}

pub fn world_min_coord() -> (f32, f32) {
    let (w, h) = world_size();
    (-w / TWO, -h / TWO)
}

pub fn world_max_coord() -> (f32, f32) {
    let (w, h) = world_size();
    (w / TWO, h / TWO)
}

pub fn world_size() -> (f32, f32) {
    let (w, h) = screen_size();
    (w * THREE, h * THREE)
}

pub fn world_center() -> (f32, f32) {
    (0.0, 0.0)
}

pub fn get_world_mouse_pos(camera: &Camera2D) -> Vec2 {
    camera.screen_to_world(mouse_position().into())
}

pub fn crop_to_world(pos: Point<Real>) -> Point<Real> {
    let (w, h) = world_size();
    let (hw, hh) = (w / TWO, h / TWO);
    let pos: XY<Real> = *pos;
    let (mut x, mut y) = (pos.x, pos.y);

    x = (x - hw).rem_euclid(w) - hw;
    y = (y - hh).rem_euclid(h) - hh;

    point!(x, y)
}

pub fn make_camera() -> Camera2D {
    let (w, h) = screen_center();

    Camera2D {
        target: world_center().into(),
        zoom: vec2(1.0 / w, -1.0 / h),
        offset: vec2(0.0, 0.0),
        rotation: 0.0,

        render_target: None,
        viewport: None,
    }
}
