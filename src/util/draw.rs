use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::{self, vec2, vec3, Color, Vec2, Vec3, Vec3Swizzles, RED};

use nalgebra::{point, vector};
use once_cell::sync::Lazy;
use rapier2d::math::{Isometry, Real};
use rapier2d::prelude::{Collider, ColliderSet, RigidBody};

use crate::util::screen::world_size;

use super::resource::Resource;
use super::screen::TWO;

fn offsets() -> Vec<Vec2> {
    let (w, h) = world_size();

    vec![
        vec2(-w, -h),
        vec2(w, -h),
        vec2(-w, h),
        vec2(w, h),
        vec2(0.0, -h),
        vec2(0.0, h),
        vec2(-w, 0.0),
        vec2(w, 0.0),
        vec2(0.0, 0.0),
    ]
}

pub static OFFSETS: Lazy<Vec<Vec2>> = Lazy::new(offsets);

fn draw_collider(collider: Option<&Collider>) -> Option<()> {
    let polygon = collider?.shape().as_compound()?;

    let transform = collider?.position();

    let points = polygon
        .shapes()
        .iter()
        .filter_map(|v| Some(v.1.as_convex_polygon()?.points().to_vec()))
        .flat_map(|v| {
            v.iter()
                .map(|v| transform.transform_point(v))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let length = points.len();

    points.iter().enumerate().for_each(|(idx, p1)| {
        let p2 = points[(idx + 1) % length];
        draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, RED);
    });

    Some(())
}

pub fn draw_colliders(rigid_body: &RigidBody, collider_set: &ColliderSet) {
    rigid_body.colliders().iter().for_each(|c| {
        draw_collider(collider_set.get(*c));
    })
}

pub fn draw(resource: &Resource, transform: Isometry<Real>) {
    let offset = vector!(resource.width / TWO, resource.height / TWO);

    let mesh = &resource.mesh;

    let vertices = mesh
        .vertices
        .iter()
        .map(|v| -> Vertex {
            let xy = v.position.xy();
            let xy = point!(xy.x, xy.y);
            let xy = xy - offset;
            let xy = transform.transform_point(&xy);

            Vertex {
                position: Vec3::new(xy[0], xy[1], v.position.z),
                uv: v.uv,
                color: v.color,
            }
        })
        .collect::<Vec<_>>();

    OFFSETS.iter().for_each(|offset| {
        let indices = mesh.indices.clone();
        let texture = mesh.texture.clone();

        let new_vert = vertices
            .iter()
            .map(|v| {
                let original = v.position;
                let pos = original.xy() + *offset;
                Vertex {
                    position: vec3(pos.x, pos.y, original.z),
                    uv: v.uv,
                    color: v.color,
                }
            })
            .collect::<Vec<_>>();

        draw_mesh(&Mesh {
            vertices: new_vert,
            indices,
            texture,
        });
    });
}

pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) -> Option<()> {
    OFFSETS.iter().for_each(|offset| {
        let (o_x, o_y) = (*offset).into();

        prelude::draw_line(x1 + o_x, y1 + o_y, x2 + o_x, y2 + o_y, thickness, color);
    });

    Some(())
}
