extern crate amethyst;

use amethyst::{
    core::{
        bundle::SystemBundle,
        frame_limiter::FrameRateLimitStrategy,
        shrev::{EventChannel, ReaderId},
    },
    ecs::{DispatcherBuilder, Read, Resources, System, SystemData, World, Write},
    prelude::*,
};

use amethyst::Error;
use core::result::Result;

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PopulationGrowthSystem, "population_system", &[]);
        Ok(())
    }
}


struct GameplayState;

struct PopulationGrowthSystem;

impl<'a> System<'a> for PopulationGrowthSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {

    }
}


impl SimpleState for GameplayState {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let world = World::new();

    let game_data = GameDataBuilder::default().with_bundle(MyBundle)?;

    let mut game = Application::build("./", GameplayState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
        .build(game_data)?;

    game.run();
    Ok(())
}

