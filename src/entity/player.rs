use macroquad::prelude::{get_time, is_key_down, is_mouse_button_down};
use miniquad::{KeyCode, MouseButton};

use crate::{util::screen::screen_size, world::world::World};

use super::{
    drawable::Drawable,
    entity::{Entity, EntityHolder},
    physics::PhysicsLike,
    projectile::projectile::{Projectile, ProjectileLike},
};

#[derive(Clone, Copy)]
pub enum MapArea {
    UpLeft { offset_x: f32, offset_y: f32 },
    UpRight { offset_x: f32, offset_y: f32 },
    DownLeft { offset_x: f32, offset_y: f32 },
    DownRight { offset_x: f32, offset_y: f32 },
}

pub struct Player {
    pub last_fire_time: f64,
}

pub trait PlayerLike {
    fn update_input(&mut self) -> Option<()>;

    //fn area_of_map(&self) -> Option<MapArea>;

    fn update_fire(&mut self, player: &EntityHolder, drawable: &Drawable) -> Option<Entity>;
}

impl PlayerLike for Entity {
    fn update_input(&mut self) -> Option<()> {
        self.update_rotation()?;
        self.update_velocity()?;

        Some(())
    }

    /*
    fn area_of_map(&self) -> Option<MapArea> {
        let (w, h) = screen_size();

        let (x, y) = self.pos()?.into();
        let (x, y) = (x / w, y / h);

        let area = if x < 0.0 {
            if y >= 0.0 {
                MapArea::DownLeft {
                    offset_x: -w,
                    offset_y: h,
                }
            } else {
                MapArea::UpLeft {
                    offset_x: -w,
                    offset_y: -h,
                }
            }
        } else {
            if y >= 0.0 {
                MapArea::DownRight {
                    offset_x: w,
                    offset_y: h,
                }
            } else {
                MapArea::UpRight {
                    offset_x: w,
                    offset_y: -h,
                }
            }
        };

        Some(area)
    }
     */

    fn update_fire(&mut self, player: &EntityHolder, drawable: &Drawable) -> Option<Entity> {
        let ref mut last_fire_time = self.player.as_mut()?.last_fire_time;

        if is_mouse_button_down(MouseButton::Right) {
            let current_time = get_time();

            if (current_time - *last_fire_time) > 0.2 {
                *last_fire_time = current_time;

                return Some(Entity::spawn_projectile(*player, self, 1.0, drawable)?);
            }
        }

        None
    }
}

impl Entity {
    fn update_velocity(&mut self) -> Option<()> {
        let ref mut velocity = self.physics.as_mut()?.velocity;

        if is_key_down(KeyCode::S) {
            *velocity *= 0.97;
        }
        if is_key_down(KeyCode::W) {
            *velocity += 0.15;
        }

        Some(())
    }

    fn update_rotation(&mut self) -> Option<()> {
        let ref mut rotation = self.physics.as_mut()?.rotation;

        if is_key_down(KeyCode::D) {
            *rotation += 0.025;
        }
        if is_key_down(KeyCode::A) {
            *rotation -= 0.025;
        }

        Some(())
    }
}
