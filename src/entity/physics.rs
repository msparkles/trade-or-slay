use macroquad::prelude::{vec2, Vec2};

use crate::util::screen::crop_to_world;

use super::entity::{Entity, EntityHolder};

pub struct Physics {
    pub pos: Vec2,
    pub velocity: f32,
    pub rotation: f32,
}

impl Default for Physics {
    fn default() -> Self {
        let pos = vec2(0.0, 0.0);
        let velocity = 0.0;
        let rotation = 0.0;

        Self {
            pos,
            velocity,
            rotation,
        }
    }
}
pub trait PhysicsLike {
    fn pos(&self) -> Option<Vec2>;
    fn velocity(&self) -> Option<f32>;
    fn rotation(&self) -> Option<f32>;

    fn angle_to(&self, pos: Vec2) -> Option<f32>;
    fn rotation_to_unit_vector(&self) -> Option<Vec2>;
    fn update_entity_position(&mut self) -> Option<()>;
}

impl PhysicsLike for Entity {
    fn pos(&self) -> Option<Vec2> {
        Some(self.physics.as_ref()?.pos)
    }

    fn velocity(&self) -> Option<f32> {
        Some(self.physics.as_ref()?.velocity)
    }

    fn rotation(&self) -> Option<f32> {
        Some(self.physics.as_ref()?.rotation)
    }

    fn angle_to(&self, pos: Vec2) -> Option<f32> {
        let d = pos - self.pos()?;

        let mut r = d.y.atan2(d.x);
        if r.is_nan() {
            r = 0.0;
        }

        return Some(r);
    }

    fn rotation_to_unit_vector(&self) -> Option<Vec2> {
        Some((self.rotation()?.cos(), self.rotation()?.sin()).into())
    }

    fn update_entity_position(&mut self) -> Option<()> {
        let d_pos = self.rotation_to_unit_vector()? * self.velocity()?;

        let ref mut pos = self.physics.as_mut()?.pos;

        // velocity
        *pos += d_pos;

        // wrap
        let (ref mut x, ref mut y) = (*pos).into();
        crop_to_world(x, y);

        // update
        *pos = vec2(*x, *y);

        Some(())
    }
}
