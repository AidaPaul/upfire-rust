const BORONITE: i8 = 1;
const CORBOMITE: i8 = 2;
const CORUNDIUM: i8 = 3;
const DURANIUM: i8 = 4;
const GALLICITE: i8 = 5;
const MERCASSIUM: i8 = 6;
const NEUTRONIUM: i8 = 7;
const SORIUM: i8 = 8;
const TRITANIUM: i8 = 9;
const URIDIUM: i8 = 10;
const VENDARITE: i8 = 11;

extern crate amethyst;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use amethyst::{
    ecs::{Entities, Join, NullStorage, ReadStorage, System, World, WriteStorage},
    prelude::*,
};

#[derive(Default)]
pub struct Planet;

impl Component for Planet {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct Population {
    count: u32,
}

impl Component for Population {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Temperature {
    value: f64,
}

impl Component for Temperature {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Consistency {
    oxygen: f64,
    nitrogen: f64,
    co2: f64,
}

#[derive(Debug)]
pub struct Atmosphere {
    consistency: Consistency,
    pressure: f64,
}

impl Component for Atmosphere {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Resource {
    resource_type: i8,
    amount: u32,
    difficulty: i8,
}

impl Component for Resource {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Mine {
        efficiency: u32,
        input_type: i8,
        output_type: i8,
        capacity: u32,
        capacity_max: u32,
}

impl Component for Mine {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct AutomatedMine {
        efficiency: u32,
        input_type: i8,
        output_type: i8,
}

impl Component for AutomatedMine {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Housing {
        capacity: u32,
        capacity_max: u32,
        quality: i8,
}

impl Component for Housing {
    type Storage = DenseVecStorage<Self>;
}


pub struct PlanetsSystem;

impl<'a> System<'a> for PlanetsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Planet>);

    fn run(&mut self, (entities, mut planet): Self::SystemData) {
        for (e, mut planet) in (&*entities, &mut planet).join() {

        }
    }
}

pub struct PlanetaryGrowth;

impl<'a> System<'a> for PlanetaryGrowth {
    type SystemData = (Entities<'a>, WriteStorage<'a, Population>);

    fn run(&mut self, (entities, mut population): Self::SystemData) {
        for (e, mut population) in (&*entities, &mut population).join() {

        }
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
        {

        }
    }
}

pub struct MiningSystem;

impl<'a> System<'a> for MiningSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Mine>, WriteStorage<'a, Resource>);

    fn run(&mut self, (entities, mut mine, mut resource): Self::SystemData) {
        for (e, mine, mut resource) in (&*entities, &mine, &mut resource).join() {
        }
    }
}

pub struct AutomatedMiningSystem;

impl<'a> System<'a> for AutomatedMiningSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, AutomatedMine>, WriteStorage<'a, Resource>);

    fn run(&mut self, (entities, mut mine, mut resource): Self::SystemData) {
        for (e, mine, mut resource) in (&*entities, &mine, &mut resource).join() {
            println!("Automated mining {:?} resource {:?}", mine, resource);
            if resource.resource_type != mine.input_type {break};
            if resource.amount == 0 {break};
            if mine.efficiency >= resource.amount {
                resource.amount = 0;
            } else {
                resource.amount = resource.amount - mine.efficiency;
            }
        }
    }
}


pub struct HousingSystem;

impl<'a> System<'a> for HousingSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Housing>, ReadStorage<'a, Population>);

    fn run(&mut self, (entities, mut housing, mut population): Self::SystemData) {
        for (e, mut housing, population) in (&*entities, &mut housing, &population).join() {

        }
    }
}

pub fn initialize_planet(world: &mut World) {
    world
        .create_entity()
        .with(Planet)
        .with(Population { count: 1000000 })
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
            amount: 300000,
            difficulty: 7,
        })
        .with(Mine {
            efficiency: 100,
            input_type: BORONITE,
            output_type: BORONITE,
            capacity: 0,
            capacity_max: 100,
        })
        .with(AutomatedMine {
            efficiency: 100,
            input_type: BORONITE,
            output_type: BORONITE,
        })
        .with(Housing {
            capacity: 1000000,
            capacity_max: 2000000,
            quality: 50,
        })
        .build();
}
