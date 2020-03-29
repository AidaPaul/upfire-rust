extern crate amethyst;
#[macro_use]
extern crate log;

use core::result::Result;

use amethyst::core::transform::TransformBundle;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::Error;
use amethyst::{
    core::{bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy},
    ecs::DispatcherBuilder,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::states::main_game::MainGame;
use crate::systems::mining::{EmptyDepositRemovalSystem, MiningSystem};
use crate::systems::movement::*;
use crate::systems::overlay::*;

mod components;
mod states;
mod systems;

pub const ARENA_HEIGHT: f32 = 1024.0;
pub const ARENA_WIDTH: f32 = 1024.0;

#[derive(Debug)]
struct PlanetaryBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PlanetaryBundle {
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
        builder.add(UpdateOverlay, "update_overlay", &[]);
        builder.add(ControlTimeScale, "control_time_scale", &[]);
        builder.add(DebugLinesSystem, "debug_lines_system", &[]);
        Ok(())
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let _world = World::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(PlanetaryBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?;
    let assets_dir = app_root.join("assets");

    let mut game = Application::build(assets_dir, MainGame)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .build(game_data)?;

    game.run();
    Ok(())
}
