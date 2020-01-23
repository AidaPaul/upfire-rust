extern crate amethyst;
use amethyst::core::{Time, Transform};

use amethyst::ecs::*;

use crate::components::planets::*;

pub struct OrbitalMovementSystem;

impl<'a> System<'a> for OrbitalMovementSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        Read<'a, Time>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (velocities, time, mut positions, mut transforms): Self::SystemData) {
        for (velocity, position, transform) in (&velocities, &mut positions, &mut transforms).join()
        {
            position.x = 0.0 + position.angle.cos() * position.r;
            position.y = 0.0 + position.angle.sin() * position.r;
            debug!("({}, {})", position.x, position.y);
            transform.set_translation_x(position.x);
            transform.set_translation_y(position.y);
            position.angle -= velocity.angle * time.delta_seconds();
        }
    }
}
