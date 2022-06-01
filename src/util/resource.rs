use std::collections::{HashMap, HashSet};

use json::JsonValue;
use lyon::{
    lyon_tessellation::{FillTessellator, StrokeTessellator, VertexBuffers},
    path::Path,
};
use macroquad::{file, models::Mesh, prelude::vec2};

use nalgebra::{point, Point2};
use once_cell::{sync::Lazy, sync::OnceCell};
use rapier2d::{
    math::{Isometry, Real},
    prelude::{
        ActiveEvents, Collider, ColliderBuilder, InteractionGroups, RigidBody, RigidBodyBuilder,
        SharedShape,
    },
};

use roxmltree::Document;
use usvg::{Node, NodeExt, NodeKind, Options, Tree};

use crate::util::screen::TWO;

use super::{
    collision::Collision,
    resource_util::{fill, paths, stroke, Geometry},
};

#[derive(Clone, Debug)]
pub struct FirePoints {
    pub points: Vec<[f32; 2]>,
    point2s: OnceCell<Vec<Point2<Real>>>,
}

impl FirePoints {
    pub fn new(points: Vec<[f32; 2]>) -> Self {
        Self {
            points,
            point2s: OnceCell::new(),
        }
    }

    pub fn get_point2s(&self) -> &Vec<Point2<Real>> {
        self.point2s.get_or_init(|| {
            self.points
                .clone()
                .iter()
                .map(|v| point!(v[0], v[1]))
                .collect()
        })
    }
}

#[derive(Clone, Debug)]
pub struct Attributes {
    collision_group: Option<InteractionGroups>,
    fire_points: Option<FirePoints>,
}

#[derive(Clone)]
pub struct Info {
    pub attributes: Option<Attributes>,

    pub collider: Option<Collider>,
    pub rigid_body: Option<RigidBody>,
}

impl Info {
    pub fn collision_group(&self) -> Option<&InteractionGroups> {
        self.attributes.as_ref()?.collision_group.as_ref()
    }

    pub fn fire_points(&self) -> Option<&FirePoints> {
        self.attributes.as_ref()?.fire_points.as_ref()
    }
}

pub struct Resource {
    pub mesh: Mesh,
    pub width: Real,
    pub height: Real,

    pub info: Info,
}

static OPT: Lazy<Options> = Lazy::new(svg_option);
static HIDDEN_ELEMENTS: Lazy<HashSet<String>> =
    Lazy::new(|| HashSet::from(["collider".to_string()]));

fn svg_option() -> Options {
    let opt = Options::default();

    return opt;
}

fn tessellation_build(
    p: &usvg::Path,
    mesh: &mut Geometry,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
) {
    let mut builder = Path::svg_builder();

    paths(p, &mut builder);
    let path = builder.build();

    fill(mesh, p, &path, fill_tess);
    stroke(mesh, p, &path, stroke_tess);
}

fn tessellation_single(node: &Node) -> Geometry {
    let mut fill_tess = FillTessellator::new();
    let mut stroke_tess = StrokeTessellator::new();
    let mut mesh: Geometry = VertexBuffers::new();

    if let NodeKind::Path(ref p) = *node.borrow() {
        tessellation_build(p, &mut mesh, &mut fill_tess, &mut stroke_tess)
    }

    return mesh;
}

fn tessellation(nodes: &HashMap<String, Node>) -> Geometry {
    let mut fill_tess = FillTessellator::new();
    let mut stroke_tess = StrokeTessellator::new();
    let mut mesh: Geometry = VertexBuffers::new();

    for (_, node) in nodes {
        if let NodeKind::Path(ref p) = *node.borrow() {
            tessellation_build(p, &mut mesh, &mut fill_tess, &mut stroke_tess)
        }
    }

    return mesh;
}

fn get_attributes(elements: &HashMap<String, roxmltree::Node>) -> Option<Attributes> {
    let attributes = elements
        .get("attributes")?
        .children()
        .filter_map(|v| v.text())
        .collect::<String>();

    let attributes = json::parse(&attributes).unwrap();

    let collision_group = attributes["collision_group"].as_str();
    let collision_group = collision_group.and_then(|v| Some(*Collision::from_str(v)));

    let fire_points = if let JsonValue::Array(fire_points) = &attributes["fire_points"] {
        Some(FirePoints::new(
            fire_points
                .chunks_exact(2)
                .filter_map(|v| Some([v[0].as_f32()?, v[1].as_f32()?]))
                .collect::<Vec<_>>(),
        ))
    } else {
        None
    };

    Some(Attributes {
        collision_group,
        fire_points,
    })
}

fn get_collider(
    nodes: &HashMap<String, Node>,
    attributes: Option<&Attributes>,
) -> Option<Collider> {
    let path = tessellation_single(nodes.get("collider")?);

    let vertices = path
        .vertices
        .into_iter()
        .map(|v| vec2(v.position.x, v.position.y))
        .collect::<Vec<_>>();

    let vertices = vertices
        .into_iter()
        .map(|p| point!(p.x, p.y))
        .collect::<Vec<_>>();

    let vertices = path
        .indices
        .into_iter()
        .map(|v| vertices[v as usize])
        .collect::<Vec<_>>();

    let shapes = vertices
        .chunks_exact(3)
        .map(|v| {
            (
                Isometry::identity(),
                SharedShape::convex_hull(&[v[0], v[1], v[2]]).unwrap(),
            )
        })
        .collect();

    let mut result = if let Some(collision_group) = attributes?.collision_group {
        ColliderBuilder::compound(shapes)
            .collision_groups(collision_group)
            .solver_groups(collision_group)
            .build()
    } else {
        ColliderBuilder::compound(shapes).build()
    };

    let aabb = result.compute_aabb();
    let offset = (aabb.mins - aabb.maxs) / TWO;
    result.set_position(offset.into());

    result.set_active_events(ActiveEvents::all());

    Some(result)
}

pub async fn load_resource(path: &str) -> Resource {
    log::info!("loading resource at {}", path);

    let file = file::load_string(path).await.unwrap();
    let file = Document::parse(&file).unwrap();

    let elements = file
        .descendants()
        .filter_map(|n| Some((n.attribute("id")?.to_string(), n)))
        .collect::<HashMap<_, _>>();

    let tree = Tree::from_xmltree(&file, &OPT.to_ref()).unwrap();
    let nodes = tree
        .root()
        .descendants()
        .filter_map(|n| {
            let id = n.id().to_string();
            Some((id, n))
        })
        .collect::<HashMap<_, _>>();

    let drawn = nodes
        .clone()
        .into_iter()
        .filter(|v| HIDDEN_ELEMENTS.contains(&v.0) == false)
        .collect();

    let mesh = tessellation(&drawn);
    let mesh = Mesh {
        vertices: mesh.vertices,
        indices: mesh.indices,
        texture: None,
    };

    let attributes = get_attributes(&elements);
    let collider = get_collider(&nodes, attributes.as_ref());

    let rigid_body = Some(RigidBodyBuilder::new_dynamic().build());

    let info = Info {
        attributes,
        collider,
        rigid_body,
    };

    let (width, height) = tree.svg_node().size.to_screen_size().dimensions();
    let (width, height) = (width as f32, height as f32);

    Resource {
        mesh,
        width,
        height,
        info,
    }
}
