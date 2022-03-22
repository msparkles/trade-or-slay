use macroquad::{camera::Camera2D, prelude::vec2, window::*};

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

pub fn crop_to_world(x: &mut f32, y: &mut f32) {
    let (w, h) = world_size();
    let (hw, hh) = (w / TWO, h / TWO);

    *x = (*x - hw).rem_euclid(w) - hw;

    *y = (*y - hh).rem_euclid(h) - hh;
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
