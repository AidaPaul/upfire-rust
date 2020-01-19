extern crate amethyst;

use amethyst::ecs::*;

use crate::components::planets::*;

pub struct OrbitalMovementSystem;

impl<'a> System<'a> for OrbitalMovementSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocities, mut positions): Self::SystemData) {
        for (velocity, position) in (&velocities, &mut positions).join() {
            position.x = 0.0 + position.angle.cos() * position.r;
            position.y = 0.0 + position.angle.sin() * position.r;
            info!("({}, {})", position.x, position.y);
            position.angle += velocity.angle;
        }
    }
}
