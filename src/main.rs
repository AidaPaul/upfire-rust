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
use amethyst::ecs::{Component, VecStorage, DenseVecStorage, FlaggedStorage};

use amethyst::Error;
use core::result::Result;

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        Ok(())
    }
}


pub struct Population {
    value: u32,
}

impl Component for Population {
    type Storage = FlaggedStorage<Self>;
}

pub struct Temperature {
    value: i16,
}

impl Component for Temperature {
    type Storage = FlaggedStorage<Self>;
}

struct Consistency {
    oxygen: f32,
    nitrogen: f32,
}

pub struct Atmosphere {
    consistency: Consistency,
    pressure: f32,
}

impl Component for Atmosphere {
    type Storage = VecStorage<Self>;
}

pub struct Resource {
    resource_type: i8,
    amount: u32,
    difficulty: i8,
}

impl Component for Resource {
    type Storage = FlaggedStorage<Self>;
}

enum Building {
    Mine {efficiency: f32, input_type: i8, output_type: i8, capacity: u32, capacity_max: u32},
    AutomatedMine {efficiency: f32, input_type: i8, output_type: i8},
    Housing {capacity: u32, capacity_max: u32, quality: i8},
}

impl Component for Building {
    type Storage = DenseVecStorage<Self>;
}

struct GameplayState;

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

