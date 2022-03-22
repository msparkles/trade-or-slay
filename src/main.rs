mod entity;
mod info;
mod util;
mod world;

#[macro_use]
extern crate lazy_static;

use entity::{drawable::Drawable, entity::Entity, physics::Physics, player::Player};
use macroquad::prelude::*;
use miniquad::conf::Conf;

use util::{
    screen::{make_camera, world_max_coord, world_min_coord},
    texture,
};
use world::world::World;

const TEXTURE_SHIP: &str = "textures/ship.svg";
const TEXTURE_BULLET: &str = "textures/bullet.svg";
//const TEXTURE_CURSOR: &str = "textures/cursor.svg";

fn config() -> Conf {
    let mut conf = Conf::default();
    conf.window_title = "Trade or Slay".to_string();
    conf.window_width = 1920;
    conf.window_height = 1080;
    conf.window_resizable = false;
    //conf.fullscreen = true;

    return conf;
}

#[macroquad::main(config)]
async fn main() {
    // pre-init

    let opt = usvg::Options::default();

    let ship = texture::load_texture(TEXTURE_SHIP, &opt);
    let bullet = texture::load_texture(TEXTURE_BULLET, &opt);
    //let cursor = texture::load_texture(TEXTURE_CURSOR, &opt);

    // init
    let mut world = World::default();
    let player = Entity {
        physics: Some(Physics::default()),
        drawable: Some(Drawable { texture: ship }),
        player: Some(Player {
            last_fire_time: 0.0,
        }),
        projectile: None,
    };
    world.set_player(player);

    loop {
        let mut camera = make_camera();

        clear_background(BLANK);

        let (ul_x, ul_y) = world_min_coord();
        let (dr_x, dr_y) = world_max_coord();

        world.update(&mut camera, &Drawable { texture: bullet });

        set_camera(&camera);

        draw_text("O", 0.0, 0.0, 80.0, GRAY);
        draw_text("UL", ul_x, ul_y, 80.0, GRAY);
        draw_text("DR", dr_x, dr_y, 80.0, GRAY);

        let (fps_x, fps_y) = camera.screen_to_world((20.0, 60.0).into()).into();
        draw_text(
            &macroquad::time::get_fps().to_string(),
            fps_x,
            fps_y,
            60.0,
            GRAY,
        );

        next_frame().await
    }
}
