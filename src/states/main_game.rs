extern crate amethyst;

use amethyst::prelude::*;

use crate::components::overlay::*;
use crate::components::planets::*;
use crate::components::structures::*;

use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::transform::Transform,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub struct MainGame;

impl SimpleState for MainGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        initialize_solar_system(world, sprite_sheet_handle);
        initialise_debug_overlay(world);
    }
}

fn initialise_debug_overlay(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/consolas.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let looking_at_transform = UiTransform::new(
        "looking_at".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -2.,
        1.,
        50.,
        14.,
    );
    let delta_time_transform = UiTransform::new(
        "delta_time".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -18.,
        1.,
        50.,
        14.,
    );

    let looking_at = world
        .create_entity()
        .with(looking_at_transform)
        .with(UiText::new(
            font.clone(),
            "Earth".to_string(),
            [1., 1., 1., 1.],
            14.,
        ))
        .build();

    let delta_time = world
        .create_entity()
        .with(delta_time_transform)
        .with(UiText::new(font, "0 ms".to_string(), [1., 1., 1., 1.], 14.))
        .build();

    world.insert(DebugOverlayText {
        looking_at,
        delta_time,
    })
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/planets.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/planets.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_solar_system(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 4,
    };

    let mut earth_transform = Transform::default();

    earth_transform.set_translation_xyz(-50.0, -50.0, 0.0);
    earth_transform.set_scale(Vector3::new(0.1, 0.1, 0.1));

    let _earth_entity = world
        .create_entity()
        .with(Deposits {
            natural: vec![Deposit {
                deposit_type: 1,
                amount: 15.0,
                difficulty: 0,
            }],
        })
        .with(Mines {
            manned: vec![
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
            ],
        })
        .with(Position {
            x: 0.0,
            y: 0.0,
            r: 100.0,
            angle: 0.0,
        })
        .with(sprite_render)
        .with(earth_transform)
        .with(Velocity { angle: 0.2 })
        .build();
}
