use macroquad::prelude::{vec2, Vec2, WHITE};

use super::{
    draw,
    screen::{world_size, TWO},
};

const ONE_HALF: f32 = 1.0 / TWO;

pub fn lines() -> Vec<(Vec2, Vec2)> {
    let lines_local = vec![
        (vec2(-ONE_HALF, -1.0), vec2(-ONE_HALF, 1.0)),
        (vec2(ONE_HALF, -1.0), vec2(ONE_HALF, 1.0)),
        (vec2(-1.0, -ONE_HALF), vec2(1.0, -ONE_HALF)),
        (vec2(-1.0, ONE_HALF), vec2(1.0, ONE_HALF)),
    ];

    let (w, h) = world_size();
    let (w, h) = (w / TWO, h / TWO);

    lines_local
        .iter()
        .map(move |line| {
            let mut a = line.0;
            let mut b = line.1;

            a.x *= w;
            b.x *= w;
            a.y *= h;
            b.y *= h;

            (a, b)
        })
        .collect()
}

pub fn draw_bg() {
    BG_LINES.iter().for_each(|line| {
        let a = line.0;
        let b = line.1;

        draw::draw_line(a.x, a.y, b.x, b.y, 1.5, WHITE);
    })
}

lazy_static! {
    static ref BG_LINES: Vec<(Vec2, Vec2)> = lines();
}
