use macroquad::prelude::rand;

use super::screen::{world_max_coord, world_min_coord};

pub fn random_place_on_map() -> (f32, f32) {
    let (min_w, min_h) = world_min_coord();
    let (max_w, max_h) = world_max_coord();

    let w = rand::gen_range(min_w, max_w);
    let h = rand::gen_range(min_h, max_h);

    (w, h)
}
