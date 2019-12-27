extern crate amethyst;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::planets::Deposit;
use crate::components::structures::{AutomatedMine, Mine};

pub struct MiningSystem;

impl<'a> System<'a> for MiningSystem {
    type SystemData = (ReadStorage<'a, Mine>, WriteStorage<'a, Deposit>);

    fn run(&mut self, (mine, mut deposit): Self::SystemData) {
        for (mine, mut deposit) in (&mine, &mut deposit).join() {
            println!("{:?}{:?}", mine, deposit);
        }
    }
}

pub struct AutomatedMiningSystem;

impl<'a> System<'a> for AutomatedMiningSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AutomatedMine>,
        WriteStorage<'a, Deposit>,
    );

    fn run(&mut self, (entities, mine, mut deposit): Self::SystemData) {
        for (_entity, mine, mut deposit) in (&*entities, &mine, &mut deposit).join() {
            if deposit.deposit_type != mine.input_type {
                break;
            };
            if deposit.amount == 0.00 {
                break;
            };
            println!("Automated mining {:?} deposit {:?}", mine, deposit);
            if mine.efficiency >= deposit.amount {
                deposit.amount = 0.00;
            } else {
                deposit.amount -= mine.efficiency;
            }
        }
    }
}
