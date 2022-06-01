mod entity;
mod info;

#[macro_use]
mod util;

mod world;

use entity::{drawable::Drawable, entity::EntityBuilder, player::Player};
use futures::FutureExt;
use info::mouse::MouseInfo;
use macroquad::prelude::*;
use miniquad::conf::Conf;

use once_cell::sync::Lazy;
use rapier2d::{
    crossbeam,
    na::vector,
    prelude::{
        BroadPhase, CCDSolver, ChannelEventCollector, IntegrationParameters, IslandManager,
        JointSet, NarrowPhase, PhysicsPipeline,
    },
};
use util::{
    resource::{self, Resource},
    screen::{make_camera, world_max_coord, world_min_coord},
};
use world::world::World;

pub const RESOURCE_SHIP: &str = "resources/ship.svg";
pub const RESOURCE_BULLET: &str = "resources/bullet.svg";
pub const RESOURCE_CURSOR: &str = "resources/cursor.svg";
pub const RESOURCE_FONTS_IOSEVKA: &str = "resources/fonts/iosevka-term-slab-regular.ttf";

pub type ReLazy = Lazy<Resource>;

pub static SHIP: ReLazy = load_resource_lazy!(RESOURCE_SHIP);
pub static BULLET: ReLazy = load_resource_lazy!(RESOURCE_BULLET);
pub static CURSOR: ReLazy = load_resource_lazy!(RESOURCE_CURSOR);

pub static IOSEVKA: Lazy<Font> = Lazy::new(|| {
    load_ttf_font(RESOURCE_FONTS_IOSEVKA)
        .now_or_never()
        .unwrap()
        .unwrap()
});
pub static TEXT_PARAM: Lazy<TextParams> = Lazy::new(text_param);

fn config() -> Conf {
    let mut conf = Conf::default();

    conf.window_title = "Trade or Slay".to_string();
    conf.window_width = 1920;
    conf.window_height = 1080;
    conf.window_resizable = false;
    //conf.fullscreen = true;

    return conf;
}

fn draw_info(world: &World, camera: &Camera2D) {
    let (ul_x, ul_y) = world_min_coord();
    let (dr_x, dr_y) = world_max_coord();

    draw_text_ex("+", -20.0, 20.0, *TEXT_PARAM);
    draw_text_ex("UL", ul_x, ul_y, *TEXT_PARAM);
    draw_text_ex("DR", dr_x, dr_y, *TEXT_PARAM);

    let (fps_x, fps_y) = camera.screen_to_world(vec2(20.0, 60.0)).into();
    draw_text_ex(&get_fps().to_string(), fps_x, fps_y, *TEXT_PARAM);

    let (entities_x, entities_y) = camera.screen_to_world(vec2(120.0, 60.0)).into();
    draw_text_ex(
        &world.entities.len().to_string(),
        entities_x,
        entities_y,
        *TEXT_PARAM,
    );
}

fn text_param() -> TextParams {
    let mut param = TextParams::default();
    param.color = GRAY;
    param.font_size = 60;
    param.font = *IOSEVKA;
    param
}

async fn load_resources() {
    resolve_all!(IOSEVKA, SHIP, BULLET, CURSOR);
}

#[macroquad::main(config)]
async fn main() {
    simple_logger::init_with_env().unwrap();

    load_resources().await;

    // pre-init
    show_mouse(false);
    let mut world = World::default();

    {
        let drawable = Drawable::from_resource(&SHIP).unwrap();

        let player = Player {
            mouse_info: MouseInfo::default(),
            last_fire_time: 0.0,
        };

        let world_mutator = EntityBuilder::new(&SHIP)
            .drawable(drawable)
            .player(player)
            .build_no_postinit();

        // init
        world.set_player(world_mutator);
    }

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0_f32, 0.0_f32];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let (contact_send, contact_recv) = crossbeam::channel::unbounded();
    let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
    let event_handler = ChannelEventCollector::new(intersection_send, contact_send);

    let mut camera = make_camera();

    loop {
        clear_background(BLANK);

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut *world.island_manager.borrow_mut(),
            &mut broad_phase,
            &mut narrow_phase,
            &mut *world.rigid_body_set.borrow_mut(),
            &mut *world.collider_set.borrow_mut(),
            &mut *world.joint_set.borrow_mut(),
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        world.update(&contact_recv, &intersection_recv, &mut camera);

        set_camera(&camera);

        draw_info(&world, &camera);

        next_frame().await
    }
}
