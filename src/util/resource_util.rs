use lyon::{
    geom::{euclid::Point2D, Point},
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, LineCap, LineJoin, StrokeOptions,
        StrokeTessellator, StrokeVertex, VertexBuffers,
    },
    path::{builder::WithSvg, path::Builder, Path},
};
use macroquad::{
    models::Vertex,
    prelude::{Color, Vec2, Vec3},
};
use rapier2d::math::Real;

pub type Geometry = VertexBuffers<Vertex, u16>;

lazy_static! {
    static ref FALLBACK_COLOR: usvg::Color = usvg::Color::black();
}

fn to_vertex<T>(pos: Point2D<Real, T>, color: usvg::Color, alpha: f32) -> Vertex {
    Vertex {
        position: Vec3::new(pos.x, pos.y, 0.0),
        uv: Vec2::ZERO,
        color: Color::new(
            (color.red as f32) / 255.0,
            (color.green as f32) / 255.0,
            (color.blue as f32) / 255.0,
            alpha,
        ),
    }
}

fn stroke_opt(s: &usvg::Stroke) -> StrokeOptions {
    let width = s.width.value() as f32;

    let line_cap = match s.linecap {
        usvg::LineCap::Butt => LineCap::Butt,
        usvg::LineCap::Square => LineCap::Square,
        usvg::LineCap::Round => LineCap::Round,
    };
    let line_join = match s.linejoin {
        usvg::LineJoin::Miter => LineJoin::Miter,
        usvg::LineJoin::Bevel => LineJoin::Bevel,
        usvg::LineJoin::Round => LineJoin::Round,
    };

    let opt = StrokeOptions::default()
        .with_line_width(width)
        .with_line_cap(line_cap)
        .with_line_join(line_join);

    return opt;
}

pub fn paths(p: &usvg::Path, builder: &mut WithSvg<Builder>) {
    p.data.iter().for_each(|seg| match seg {
        usvg::PathSegment::MoveTo { x, y } => {
            builder.move_to(Point::new(*x as f32, *y as f32));
        }
        usvg::PathSegment::LineTo { x, y } => {
            builder.line_to(Point::new(*x as f32, *y as f32));
        }
        usvg::PathSegment::CurveTo {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => {
            builder.cubic_bezier_to(
                Point::new(*x1 as f32, *y1 as f32),
                Point::new(*x2 as f32, *y2 as f32),
                Point::new(*x as f32, *y as f32),
            );
        }
        usvg::PathSegment::ClosePath => {
            builder.close();
        }
    });
}

pub fn fill(mesh: &mut Geometry, p: &usvg::Path, path: &Path, fill_tess: &mut FillTessellator) {
    if let Some(ref fill) = p.fill {
        let alpha = fill.opacity.value() as f32;

        let color = match fill.paint {
            usvg::Paint::Color(c) => c,
            _ => *FALLBACK_COLOR,
        };

        let opt = FillOptions::default();

        fill_tess
            .tessellate(
                path,
                &opt,
                &mut BuffersBuilder::new(mesh, |vertex: FillVertex| {
                    to_vertex(vertex.position(), color, alpha)
                }),
            )
            .unwrap();
    }
}

pub fn stroke(
    mesh: &mut Geometry,
    p: &usvg::Path,
    path: &Path,
    stroke_tess: &mut StrokeTessellator,
) {
    if let Some(ref stroke) = p.stroke {
        let alpha = stroke.opacity.value() as f32;

        let color = match stroke.paint {
            usvg::Paint::Color(c) => c,
            _ => *FALLBACK_COLOR,
        };

        let opt = stroke_opt(stroke);

        stroke_tess
            .tessellate(
                path,
                &opt,
                &mut BuffersBuilder::new(mesh, |vertex: StrokeVertex| {
                    to_vertex(vertex.position(), color, alpha)
                }),
            )
            .unwrap();
    }
}
