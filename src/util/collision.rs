use rapier2d::prelude::InteractionGroups;

lazy_static! {
    static ref GLOBAL: InteractionGroups = InteractionGroups::all();
    static ref SHIPS: InteractionGroups = InteractionGroups::new(0b01, 0b11);
    static ref BULLETS: InteractionGroups = InteractionGroups::new(0b10, 0b11);
}

pub struct Collision;

impl Collision {
    pub fn from_str(s: &str) -> &InteractionGroups {
        return match s.to_lowercase().as_str() {
            "ships" => &SHIPS,
            "bullets" => &BULLETS,
            _ => &GLOBAL,
        };
    }
}
