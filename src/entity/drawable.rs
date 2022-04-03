use std::collections::VecDeque;

use macroquad::prelude::{DrawTextureParams, Texture2D, RED, WHITE};
use rapier2d::{
    math::{Point, Real},
    prelude::{ColliderSet, RigidBodySet},
};

use crate::{
    util::{
        draw::{self, draw_line},
        resource::Resource,
        screen::TWO,
    },
    DEBUG,
};

use super::{entity::Entity, physics::PhysicsLike};

#[derive(Clone, Copy)]
pub struct Drawable {
    pub texture: Texture2D,
}

impl Drawable {
    pub fn from_resource(resource: &Resource) -> Option<Drawable> {
        Some(Drawable {
            texture: resource.texture,
        })
    }
}

pub trait DrawableLike {
    fn draw(&self, rigid_body_set: &RigidBodySet, collider_set: &ColliderSet) -> Option<()>;
}

impl DrawableLike for Entity {
    fn draw(&self, rigid_body_set: &RigidBodySet, collider_set: &ColliderSet) -> Option<()> {
        let ref texture = self.drawable.as_ref()?.texture;

        let mut options = DrawTextureParams::default();
        options.rotation = self.rotation(rigid_body_set)?.angle();

        let pos = *self.pos(rigid_body_set)?;
        let (x, y) = (
            pos.x - texture.width() / TWO,
            pos.y - texture.height() / TWO,
        );

        draw::draw_texture_ex(texture, x, y, WHITE, options);

        if DEBUG {
            let rigid_body = self.get_rigid_body(rigid_body_set)?;
            rigid_body.colliders().iter().for_each(|c| {
                if let Some(collider) = collider_set.get(*c) {
                    if let Some(polygon) = collider.shape().as_convex_polygon() {
                        let pos = collider.position();

                        let points: Vec<Point<Real>> = polygon
                            .points()
                            .iter()
                            .map(|p| pos.transform_point(p))
                            .collect();
                        let mut rotated = points.iter().collect::<VecDeque<_>>();
                        rotated.rotate_right(1);
                        let points = points.iter().zip(rotated);

                        points.for_each(|(p1, p2)| {
                            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, RED);
                        })
                    }
                }
            })
        }

        Some(())
    }
}
