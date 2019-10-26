extern crate amethyst;

mod systems;

use amethyst::Error;
use amethyst::{
    core::{bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy},
    ecs::DispatcherBuilder,
    prelude::*,
};
use core::result::Result;
use systems::*;

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PlanetsSystem, "planets_system", &[]);
        builder.add(PlanetaryGrowth, "planetary_growth_system", &[]);
        builder.add(PlanetaryAtmosphere, "planetary_atmosphere_system", &[]);
        builder.add(MiningSystem, "mining_system", &[]);
        builder.add(AutomatedMiningSystem, "automated_mining_system", &[]);
        builder.add(HousingSystem, "housing_system", &[]);
        Ok(())
    }
}

struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        systems::initialize_planet(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let _world = World::new();

    let game_data = GameDataBuilder::default().with_bundle(MyBundle)?;

    let mut game = Application::build("./", GameplayState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
        .build(game_data)?;

    game.run();
    Ok(())
}
