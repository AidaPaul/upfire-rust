extern crate amethyst;

use amethyst::prelude::*;

use crate::components::planets::*;
use crate::components::structures::*;

pub struct MainGame;

impl SimpleState for MainGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let _earth_entity = world
            .create_entity()
            .with(Deposits {
                natural: vec![Deposit {
                    deposit_type: 1,
                    amount: 15,
                    difficulty: 0,
                }],
            })
            .with(Mines {
                manned: vec![
                    Manned {
                        efficiency: 2,
                        input_type: 1,
                        output_type: 11,
                        capacity: 0,
                        capacity_max: 100,
                    },
                    Manned {
                        efficiency: 2,
                        input_type: 1,
                        output_type: 11,
                        capacity: 0,
                        capacity_max: 100,
                    },
                ],
            })
            .build();
    }
}
