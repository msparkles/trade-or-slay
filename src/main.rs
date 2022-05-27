mod entity;
mod info;
mod util;
mod world;

#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;

use entity::{drawable::Drawable, entity::Entity, physics::Physics, player::Player};
use info::mouse::MouseInfo;
use macroquad::prelude::*;
use miniquad::conf::Conf;

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

const RESOURCE_SHIP: &str = "resources/ship.svg";
const RESOURCE_BULLET: &str = "resources/bullet.svg";
const RESOURCE_CURSOR: &str = "resources/cursor.svg";

lazy_static! {
    static ref SHIP: Resource = resource::load_resource(RESOURCE_SHIP);
    static ref BULLET: Resource = resource::load_resource(RESOURCE_BULLET);
    static ref CURSOR: Resource = resource::load_resource(RESOURCE_CURSOR);
}

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

    draw_text("+", -20.0, 20.0, 80.0, GRAY);
    draw_text("O", 0.0, 0.0, 80.0, GRAY);
    draw_text("UL", ul_x, ul_y, 80.0, GRAY);
    draw_text("DR", dr_x, dr_y, 80.0, GRAY);

    let (fps_x, fps_y) = camera.screen_to_world(vec2(20.0, 60.0)).into();
    draw_text(&get_fps().to_string(), fps_x, fps_y, 60.0, GRAY);

    let (entities_x, entities_y) = camera.screen_to_world(vec2(120.0, 60.0)).into();
    draw_text(
        &world.entities.len().to_string(),
        entities_x,
        entities_y,
        60.0,
        GRAY,
    );
}

#[macroquad::main(config)]
async fn main() {
    simple_logger::init_with_env().unwrap();

    let _ = *SHIP;
    let _ = *BULLET;
    let _ = *CURSOR;

    // pre-init
    show_mouse(false);
    let mut world = World::default();

    let player_physics = Physics::from_resource(
        &SHIP,
        &mut world.rigid_body_set.borrow_mut(),
        &mut world.collider_set.borrow_mut(),
    );

    // init
    world.set_player(Entity {
        resource: &SHIP,
        physics: player_physics,
        drawable: Drawable::from_resource(&SHIP),
        player: Some(Player {
            mouse_info: MouseInfo::default(),
            last_fire_time: RefCell::new(0.0),
        }),
        projectile: None,
    });

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0_f32, 0.0_f32];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
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
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut world.rigid_body_set.borrow_mut(),
            &mut world.collider_set.borrow_mut(),
            &mut joint_set,
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
