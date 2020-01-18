extern crate amethyst;

use amethyst::ecs::*;

use crate::components::planets::*;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocities, mut positions): Self::SystemData) {
        for (velocity, position) in (&velocities, &mut positions).join() {
            position.x += velocity.x;
            position.y += velocity.y;
            position.z += velocity.z;
            info!("{:?}, {:?}", velocity, position);
        }
    }
}
