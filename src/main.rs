mod entity;
mod info;
mod util;
mod world;

#[macro_use]
extern crate lazy_static;

use std::{borrow::BorrowMut, cell::RefCell};

use entity::{drawable::Drawable, entity::Entity, physics::Physics, player::Player};
use info::mouse::MouseInfo;
use macroquad::prelude::*;
use miniquad::conf::Conf;

use rapier2d::{
    na::vector,
    prelude::{
        BroadPhase, CCDSolver, IntegrationParameters, IslandManager, JointSet, NarrowPhase,
        PhysicsPipeline,
    },
};
use usvg::Options;
use util::{
    resource::{self, Resource},
    screen::{make_camera, world_max_coord, world_min_coord},
};
use world::world::World;

const RESOURCE_SHIP: &str = "resources/ship.svg";
const RESOURCE_BULLET: &str = "resources/bullet.svg";
const RESOURCE_CURSOR: &str = "resources/cursor.svg";

lazy_static! {
    static ref OPT: Options = usvg::Options::default();
    static ref SHIP: Resource = resource::load_resource(RESOURCE_SHIP, &OPT);
    static ref BULLET: Resource = resource::load_resource(RESOURCE_BULLET, &OPT);
    static ref CURSOR: Resource = resource::load_resource(RESOURCE_CURSOR, &OPT);
}

const DEBUG: bool = true;

fn config() -> Conf {
    let mut conf = Conf::default();
    conf.window_title = "Trade or Slay".to_string();
    conf.window_width = 1920;
    conf.window_height = 1080;
    conf.window_resizable = false;
    //conf.fullscreen = true;

    return conf;
}

fn draw_info(camera: &Camera2D) {
    let (ul_x, ul_y) = world_min_coord();
    let (dr_x, dr_y) = world_max_coord();

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
}

#[macroquad::main(config)]
async fn main() {
    // pre-init
    show_mouse(false);
    let mut world = World::default();

    let ship_rigid_handle = world.rigid_body_set.borrow_mut().insert(
        SHIP.rigid_body
            .clone()
            .expect("ship doesn't have rigid body"),
    );
    let ship_collider_handle = world.collider_set.borrow_mut().insert_with_parent(
        SHIP.collider.clone().expect("ship doesn't have collider"),
        ship_rigid_handle,
        world.rigid_body_set.borrow_mut().borrow_mut(),
    );

    // init
    let player = Entity {
        physics: Some(Physics {
            rigid_body: ship_rigid_handle,
            collider: ship_collider_handle,
        }),
        drawable: Some(Drawable {
            texture: SHIP.texture,
        }),
        player: Some(Player {
            mouse_info: MouseInfo::default(),
            last_fire_time: RefCell::new(0.0),
        }),
        projectile: None,
    };
    world.set_player(player);

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
    let event_handler = ();

    loop {
        clear_background(BLANK);

        let mut camera = make_camera();

        world.update(&mut camera);

        set_camera(&camera);

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            world.rigid_body_set.borrow_mut().borrow_mut(),
            world.collider_set.borrow_mut().borrow_mut(),
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        draw_info(&camera);

        next_frame().await
    }
}
