use std::cell::RefCell;

use macroquad::prelude::{get_time, is_key_down, is_mouse_button_down};
use miniquad::{KeyCode, MouseButton};

use nalgebra::{vector, Complex, ComplexField, Unit};
use rapier2d::prelude::{ColliderSet, RigidBodySet};

use crate::info::mouse::MouseInfo;

use super::{
    entity::{Entity, EntityHolder},
    physics::PhysicsLike,
    projectile::projectile::ProjectileLike,
};

pub struct Player {
    pub mouse_info: MouseInfo,
    pub last_fire_time: RefCell<f64>,
}

pub trait PlayerLike {
    fn mouse_info(&self) -> Option<&MouseInfo>;
    fn mouse_info_mut(&mut self) -> Option<&mut MouseInfo>;
    fn angle_to_mouse(&self, rigid_body_set: &RigidBodySet) -> Option<f32>;

    fn update_input(&self, rigid_body_set: &mut RigidBodySet) -> Option<()>;

    fn update_fire(
        &self,
        player: &EntityHolder,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Entity>;
}

impl PlayerLike for Entity {
    fn mouse_info(&self) -> Option<&MouseInfo> {
        Some(&self.player.as_ref()?.mouse_info)
    }

    fn mouse_info_mut(&mut self) -> Option<&mut MouseInfo> {
        Some(&mut self.player.as_mut()?.mouse_info)
    }

    fn angle_to_mouse(&self, rigid_body_set: &RigidBodySet) -> Option<f32> {
        let pos = *self.pos(rigid_body_set)?;
        let mouse_pos = self.mouse_info()?.pos;
        let mouse_pos = Complex::new(mouse_pos.x - pos.x, mouse_pos.y - pos.y);
        let mouse_pos = Unit::from_complex(mouse_pos);

        Some(self.rotation(rigid_body_set)?.angle_to(&mouse_pos))
    }

    fn update_input(&self, rigid_body_set: &mut RigidBodySet) -> Option<()> {
        self.update_rotation(rigid_body_set)?;
        self.update_velocity(rigid_body_set)?;

        Some(())
    }

    fn update_fire(
        &self,
        player: &EntityHolder,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Entity> {
        let mut last_fire_time = self.player.as_ref()?.last_fire_time.borrow_mut();

        if is_mouse_button_down(MouseButton::Right) {
            let current_time = get_time();

            if (current_time - *last_fire_time) > 0.2 {
                *last_fire_time = current_time;

                return Some(Entity::spawn_projectile(
                    *player,
                    self,
                    1.0,
                    rigid_body_set,
                    collider_set,
                )?);
            }
        }

        None
    }
}

impl Entity {
    fn update_velocity<'a>(&self, rigid_body_set: &'a mut RigidBodySet) -> Option<()> {
        let rigid_body = self.get_rigid_body_mut(rigid_body_set)?;
        let mut velocity = *rigid_body.linvel();

        if is_key_down(KeyCode::S) {
            velocity = velocity.scale(0.97);
        }
        if is_key_down(KeyCode::W) {
            let d_v = rigid_body.rotation().scale(5.0);
            let d_v = vector!(d_v.re, d_v.im);

            velocity += d_v;
        }
        rigid_body.set_linvel(velocity, true);

        Some(())
    }

    fn update_rotation(&self, rigid_body_set: &mut RigidBodySet) -> Option<()> {
        let angle_to_mouse = self.angle_to_mouse(rigid_body_set)?;

        let rigid_body = self.get_rigid_body_mut(rigid_body_set)?;
        let mut rotation = rigid_body.rotation().angle();

        rotation += angle_to_mouse / 20.0;

        /*
        if is_key_down(KeyCode::D) {
            rotation += 0.025;
        }
        if is_key_down(KeyCode::A) {
            rotation -= 0.025;
        }
        */
        rigid_body.set_rotation(rotation, true);

        Some(())
    }
}
