extern crate amethyst;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use amethyst::{
    ecs::{Entities, Join, NullStorage, ReadStorage, System, World, WriteStorage},
    prelude::*,
};

use crate::components::planets::{Atmosphere, Planet, Population, Resource, Temperature};
use crate::components::structures::Housing;

pub struct PlanetsSystem;

impl<'a> System<'a> for PlanetsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Planet>);

    fn run(&mut self, (entities, mut planet): Self::SystemData) {
        for (e, mut planet) in (&*entities, &mut planet).join() {}
    }
}

pub struct PlanetaryGrowth;

impl<'a> System<'a> for PlanetaryGrowth {
    type SystemData = (Entities<'a>, WriteStorage<'a, Population>);

    fn run(&mut self, (entities, mut population): Self::SystemData) {
        for (e, mut population) in (&*entities, &mut population).join() {}
    }
}

pub struct PlanetaryAtmosphere;

impl<'a> System<'a> for PlanetaryAtmosphere {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Atmosphere>,
        WriteStorage<'a, Temperature>,
    );

    fn run(&mut self, (entities, mut atmosphere, mut temperature): Self::SystemData) {
        for (e, mut atmosphere, mut temperature) in
            (&*entities, &mut atmosphere, &mut temperature).join()
        {}
    }
}

pub struct HousingSystem;

impl<'a> System<'a> for HousingSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Housing>,
        ReadStorage<'a, Population>,
    );

    fn run(&mut self, (entities, mut housing, mut population): Self::SystemData) {
        for (e, mut housing, population) in (&*entities, &mut housing, &population).join() {}
    }
}
