/*
use std::sync::Arc;

use futures::{FutureExt, SinkExt};
use macroquad::{
    camera::Camera2D,
    input,
    prelude::{mouse_position, vec2, Vec2},
};

use pharos::*;

use crate::util::screen::{self, world_center};

#[derive(Debug, Clone)]
pub enum MouseRawEvent {
    Move { x: f32, y: f32 },
}

pub struct MouseInfoRaw {
    pharos: Pharos<Arc<MouseRawEvent>>,

    pub pos: Vec2,
}

impl Observable<Arc<MouseRawEvent>> for MouseInfoRaw {
    type Error = PharErr;

    fn observe(
        &mut self,
        options: ObserveConfig<Arc<MouseRawEvent>>,
    ) -> Observe<'_, Arc<MouseRawEvent>, Self::Error> {
        self.pharos.observe(options)
    }
}

impl Default for MouseInfoRaw {
    fn default() -> Self {
        Self {
            pharos: Pharos::default(),
            pos: world_center().into(),
        }
    }
}

/*
impl MouseInfoRaw {
    pub async fn from_mouse(&mut self, camera: &Camera2D) {
        let (x, y) = screen::get_world_mouse_pos(camera).into();

        println!("{}, {}", x, y);

        if x != self.pos.x || y != self.pos.y {
            self.pos.x = x;
            self.pos.y = y;

            self.pharos
                // TODO check this thing?
                .feed(Arc::new(MouseRawEvent::Move { x, y }))
                .now_or_never();
        }
    }
}
 */

pub struct MouseInfo {
    pub mouse_info_raw: MouseInfoRaw,

    pub events: Events<Arc<MouseRawEvent>>,
}

impl MouseInfo {
    pub fn pos(&self) -> Vec2 {
        self.mouse_info_raw.pos
    }

    pub async fn default() -> Self {
        let mut mouse_info_raw = MouseInfoRaw::default();

        let events = mouse_info_raw
            // TODO check this thing?
            .observe(Channel::Bounded(1).into())
            .await
            .expect("observing mouse event");

        Self {
            mouse_info_raw,
            events,
        }
    }
}
*/
