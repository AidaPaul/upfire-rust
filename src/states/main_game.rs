const BORONITE: i8 = 1;

extern crate amethyst;
use amethyst::prelude::*;

use crate::components::planets::*;
use crate::components::structures::*;

pub struct MainGame;

impl SimpleState for MainGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world
            .create_entity()
            .with(Planet)
            .with(Population { count: 1_000_000 })
            .with(Temperature { value: 14.6 })
            .with(Atmosphere {
                consistency: Consistency {
                    oxygen: 18.0,
                    nitrogen: 81.5,
                    co2: 0.5,
                },
                pressure: 1013.25,
            })
            .with(Resource {
                resource_type: BORONITE,
                amount: 300_000.00,
                difficulty: 7,
            })
            .with(Mine {
                efficiency: 100.00,
                input_type: BORONITE,
                output_type: BORONITE,
                capacity: 50,
                capacity_max: 100,
            })
            .with(AutomatedMine {
                efficiency: 100.00,
                input_type: BORONITE,
                output_type: BORONITE,
            })
            .with(Housing {
                capacity: 1_000_000,
                capacity_max: 2_000_000,
                quality: 50,
            })
            .build();
    }
}
