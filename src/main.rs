extern crate amethyst;
#[macro_use]
extern crate log;

use core::result::Result;

use amethyst::Error;
use amethyst::{
    core::{bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy},
    ecs::DispatcherBuilder,
    prelude::*,
};

use crate::states::main_game::MainGame;
use crate::systems::mining::{EmptyDepositRemovalSystem, MiningSystem};
use crate::systems::movement::*;

mod components;
mod states;
mod systems;

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(MiningSystem, "mining_system", &[]);
        builder.add(OrbitalMovementSystem, "orbital_movement_system", &[]);
        builder.add(
            EmptyDepositRemovalSystem,
            "empty_deposit_removal_system",
            &[],
        );
        Ok(())
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let _world = World::new();

    let game_data = GameDataBuilder::default().with_bundle(MyBundle)?;

    let mut game = Application::build("./", MainGame)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
        .build(game_data)?;

    game.run();
    Ok(())
}
