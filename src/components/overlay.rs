extern crate amethyst;
use amethyst::ecs::prelude::Entity;

#[derive(Default)]
pub struct DebugOverlay {
    pub looking_at: String,
}

pub struct DebugOverlayText {
    pub looking_at: Entity,
    pub delta_time: Entity,
    pub time_scale: Entity,
}
