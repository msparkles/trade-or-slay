use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::prelude::{get_time, is_key_down, is_mouse_button_down};

use nalgebra::{vector, Complex, ComplexField, Unit};
use rapier2d::math::Real;
use rapier2d::prelude::RigidBody;

use crate::info::mouse::MouseInfo;
use crate::world::world_mutator::WorldMutator;

use super::{
    entity::{Entity, EntityHolder},
    projectile::projectile::ProjectileLike,
};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub mouse_info: MouseInfo,
    pub last_fire_time: f64,
}

pub trait PlayerLike {
    fn mouse_info(&self) -> Option<&MouseInfo>;
    fn mouse_info_mut(&mut self) -> Option<&mut MouseInfo>;
    fn angle_to_mouse(&self, rigid_body: &RigidBody) -> Option<Real>;

    fn update_input(&self, rigid_body: &mut RigidBody) -> Option<()>;

    fn update_fire(&mut self, player: &EntityHolder) -> Option<WorldMutator>;
}

impl PlayerLike for Entity {
    fn mouse_info(&self) -> Option<&MouseInfo> {
        Some(&self.player.as_ref()?.mouse_info)
    }

    fn mouse_info_mut(&mut self) -> Option<&mut MouseInfo> {
        Some(&mut self.player.as_mut()?.mouse_info)
    }

    fn angle_to_mouse(&self, rigid_body: &RigidBody) -> Option<Real> {
        let pos = rigid_body.translation();
        let mouse_pos = self.mouse_info()?.pos;
        let mouse_pos = Complex::new(mouse_pos.x - pos.x, mouse_pos.y - pos.y);
        let mouse_pos = Unit::from_complex(mouse_pos);

        Some(rigid_body.rotation().angle_to(&mouse_pos))
    }

    fn update_input(&self, rigid_body: &mut RigidBody) -> Option<()> {
        self.update_rotation(rigid_body);
        self.update_velocity(rigid_body);

        Some(())
    }

    fn update_fire(&mut self, player: &EntityHolder) -> Option<WorldMutator> {
        let ref mut last_fire_time = self.player.as_mut()?.last_fire_time;

        if is_mouse_button_down(MouseButton::Right) {
            let current_time = get_time();

            if (current_time - *last_fire_time) > 0.2 {
                *last_fire_time = current_time;

                return Some(Entity::spawn_projectile(*player, *self, 1.0)?);
            }
        }

        None
    }
}

impl Entity {
    fn update_velocity(&self, rigid_body: &mut RigidBody) -> Option<()> {
        let mut velocity = *rigid_body.linvel();

        if is_key_down(KeyCode::S) {
            velocity = velocity.scale(0.97);
        }
        if is_key_down(KeyCode::W) {
            let d_v = rigid_body.rotation().scale(10.0);
            let d_v = vector!(d_v.re, d_v.im);

            velocity += d_v;
        }
        rigid_body.set_linvel(velocity, true);

        Some(())
    }

    fn update_rotation(&self, rigid_body: &mut RigidBody) -> Option<()> {
        let angle_to_mouse = self.angle_to_mouse(rigid_body)?;

        let angvel = angle_to_mouse * 3.0;

        /*
        if is_key_down(KeyCode::D) {
            rotation += 0.025;
        }
        if is_key_down(KeyCode::A) {
            rotation -= 0.025;
        }
        */
        rigid_body.set_angvel(angvel, true);

        Some(())
    }
}
