extern crate amethyst;
use amethyst::core::Time;
use amethyst::{ecs::prelude::*, ui::UiText};

use crate::components::overlay::*;

pub struct UpdateOverlay;

impl<'s> System<'s> for UpdateOverlay {
    type SystemData = (
        WriteStorage<'s, UiText>,
        ReadExpect<'s, DebugOverlayText>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut ui_text, overlay_text, time): Self::SystemData) {
        if let Some(delta_time) = ui_text.get_mut(overlay_text.delta_time) {
            delta_time.text = time.delta_time().as_secs_f32().to_string();
        }
        if let Some(time_scale) = ui_text.get_mut(overlay_text.time_scale) {
            time_scale.text = time.time_scale().to_string();
        }
    }
}
