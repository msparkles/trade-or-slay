use std::fs::read;

use macroquad::prelude::{Image, Texture2D};

use resvg::render;
use tiny_skia::Pixmap;
use usvg::Tree;

pub fn load_texture(path: &str, opt: &usvg::Options) -> Texture2D {
    let texture = read(path).unwrap();
    let texture = Tree::from_data(&texture, &opt.to_ref()).unwrap();
    let pixmap_size = texture.svg_node().size.to_screen_size();

    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    render(
        &texture,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    );

    let pixmap: Vec<u8> = pixmap
        .pixels()
        .into_iter()
        .flat_map(|v| [v.red(), v.green(), v.blue(), v.alpha()])
        .collect();

    Texture2D::from_image(&Image {
        bytes: pixmap,
        width: pixmap_size.width() as u16,
        height: pixmap_size.height() as u16,
    })
}
