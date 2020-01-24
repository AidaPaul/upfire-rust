extern crate amethyst;
use amethyst::ecs::prelude::Entity;

#[derive(Default)]
pub struct DebugOverlay {
    pub looking_at: String,
    pub delta_time: f32,
    pub time_scale: f32,
}

pub struct DebugOverlayText {
    pub looking_at: Entity,
    pub delta_time: Entity,
}
