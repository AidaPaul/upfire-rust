extern crate amethyst;

use amethyst::Error;
use amethyst::{
    core::{bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy},
    ecs::DispatcherBuilder,
    prelude::*,
};
use core::result::Result;
use systems::*;

mod components;
mod states;
mod systems;

use crate::components::planets::*;
use crate::components::structures::*;
use crate::states::main_game::MainGame;
use crate::systems::mining::{AutomatedMiningSystem, MiningSystem};
use crate::systems::planets::{HousingSystem, PlanetaryAtmosphere, PlanetaryGrowth, PlanetsSystem};

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(PlanetsSystem, "planets_system", &[]);
        builder.add(PlanetaryGrowth, "planetary_growth_system", &[]);
        builder.add(PlanetaryAtmosphere, "planetary_atmosphere_system", &[]);
        builder.add(MiningSystem, "mining_system", &[]);
        builder.add(AutomatedMiningSystem, "automated_mining_system", &[]);
        builder.add(HousingSystem, "housing_system", &[]);
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
