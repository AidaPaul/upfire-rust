extern crate amethyst;

use amethyst::prelude::*;

use crate::components::planets::*;
use crate::components::structures::*;

pub struct MainGame;

impl SimpleState for MainGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let earth_entity = world
            .create_entity()
            .with(generate_planet("Earth".to_string()))
            .build();
        let deposit_entity = world
            .create_entity()
            .with(Deposit {
                deposit_type: 1,
                amount: 300_000.0,
                difficulty: 0,
            })
            .build();
        world
            .write_storage::<Planet>()
            .get_mut(earth_entity)
            .unwrap()
            .deposits
            .push(deposit_entity);
        let mine_entity = world
            .create_entity()
            .with(Mine {
                efficiency: 100.0,
                input_type: 1,
                output_type: 0,
                capacity: 50,
                capacity_max: 200,
            })
            .build();
        world
            .write_storage::<Planet>()
            .get_mut(earth_entity)
            .unwrap()
            .mines
            .push(mine_entity);
    }
}
