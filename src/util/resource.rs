use std::fs::read;

use macroquad::prelude::{Image, Texture2D};

use nalgebra::point;
use rapier2d::{
    math::{Isometry, Point, Real},
    prelude::{ActiveEvents, Collider, ColliderBuilder, RigidBody, RigidBodyBuilder},
};
use resvg::render;
use tiny_skia::Pixmap;
use usvg::{Node, NodeKind, Tree};

use super::screen::TWO;

pub struct Resource {
    pub texture: Texture2D,
    pub collider: Option<Collider>,
    pub rigid_body: Option<RigidBody>,
}

fn convert_path(width: Real, height: Real, path: &usvg::PathData) -> Option<(Collider, RigidBody)> {
    let mut c_y: Real = 0.0;
    let mut c_x: Real = 0.0;
    let mut points: Vec<Point<Real>> = vec![];

    for seg in path.iter() {
        match *seg {
            usvg::PathSegment::MoveTo { x, y } => {
                c_x = x as Real;
                c_y = y as Real;
            }
            usvg::PathSegment::LineTo { x, y } => {
                points.push(point!(c_x, c_y));
                c_x = x as Real;
                c_y = y as Real;
            }
            usvg::PathSegment::ClosePath => {
                points.push(point!(c_x, c_y));
            }
            _ => {}
        }
    }
    let pos = Isometry::translation(-width / TWO, -height / TWO);
    let points: Vec<Point<_>> = points.iter().map(|v| pos.transform_point(v)).collect();

    Some((
        ColliderBuilder::convex_polyline(points)?.build(),
        RigidBodyBuilder::new_dynamic().build(),
    ))
}

fn to_info(width: Real, height: Real, node: Option<Node>) -> Option<(Collider, RigidBody)> {
    if node.is_some() {
        let node = node.unwrap();
        let node = &*node.borrow();
        if let NodeKind::Path(ref path) = node {
            return convert_path(width, height, &path.data);
        }
    }
    None
}

pub fn load_resource(path: &str, opt: &usvg::Options) -> Resource {
    let file = read(path).unwrap();
    let file = Tree::from_data(&file, &opt.to_ref()).unwrap();
    let pixmap_size = file.svg_node().size.to_screen_size();
    let (width, height) = (pixmap_size.width(), pixmap_size.height());

    let info = to_info(width as Real, height as Real, file.node_by_id("hitbox"));

    let mut pixmap = Pixmap::new(width, height).unwrap();
    render(
        &file,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    );

    let pixmap = pixmap
        .pixels()
        .iter()
        .flat_map(|v| [v.red(), v.green(), v.blue(), v.alpha()])
        .collect();

    let texture = Texture2D::from_image(&Image {
        bytes: pixmap,
        width: pixmap_size.width() as u16,
        height: pixmap_size.height() as u16,
    });

    if let Some((mut collider, rigid_body)) = info {
        collider.set_active_events(ActiveEvents::all());

        Resource {
            texture,
            collider: Some(collider),
            rigid_body: Some(rigid_body),
        }
    } else {
        Resource {
            texture,
            collider: None,
            rigid_body: None,
        }
    }
}
