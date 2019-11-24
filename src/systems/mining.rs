extern crate amethyst;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use amethyst::{
    ecs::{Entities, Join, NullStorage, ReadStorage, System, World, WriteStorage},
    prelude::*,
};

use crate::components::planets::Resource;
use crate::components::structures::{AutomatedMine, Mine};

pub struct MiningSystem;

impl<'a> System<'a> for MiningSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Mine>,
        WriteStorage<'a, Resource>,
    );

    fn run(&mut self, (entities, mut mine, mut resource): Self::SystemData) {
        for (e, mine, mut resource) in (&*entities, &mine, &mut resource).join() {
            if resource.resource_type != mine.input_type {
                break;
            };
            if resource.amount == 0.00 {
                break;
            };
            let manned_percentage = mine.capacity as f64 / mine.capacity_max as f64;
            let efficiency = mine.efficiency as f64 * manned_percentage;
            println!(
                "Human mining {:?} resource {:?} with efficiency {}",
                mine, resource, efficiency
            );
            if efficiency >= resource.amount {
                resource.amount = 0.00;
            } else {
                resource.amount = resource.amount - efficiency;
            }
        }
    }
}

pub struct AutomatedMiningSystem;

impl<'a> System<'a> for AutomatedMiningSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AutomatedMine>,
        WriteStorage<'a, Resource>,
    );

    fn run(&mut self, (entities, mut mine, mut resource): Self::SystemData) {
        for (e, mine, mut resource) in (&*entities, &mine, &mut resource).join() {
            if resource.resource_type != mine.input_type {
                break;
            };
            if resource.amount == 0.00 {
                break;
            };
            println!("Automated mining {:?} resource {:?}", mine, resource);
            if mine.efficiency >= resource.amount {
                resource.amount = 0.00;
            } else {
                resource.amount = resource.amount - mine.efficiency;
            }
        }
    }
}
