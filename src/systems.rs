extern crate amethyst;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use amethyst::{
    ecs::{System,
        Entities, Join, NullStorage,
        World, WriteStorage,
    },
    prelude::*,
};

#[derive(Default)]
pub struct Planet;

impl Component for Planet {
    type Storage = NullStorage<Self>;
}

pub struct Population {
    count: u32,
}

impl Component for Population {
    type Storage = FlaggedStorage<Self>;
}

pub struct Temperature {
    value: f32,
}

impl Component for Temperature {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Consistency {
    oxygen: f32,
    nitrogen: f32,
}

#[derive(Debug)]
pub struct Atmosphere {
    consistency: Consistency,
    pressure: f32,
}

impl Component for Atmosphere {
    type Storage = VecStorage<Self>;
}

pub struct Resource {
    resource_type: i8,
    amount: u32,
    difficulty: i8,
}

impl Component for Resource {
    type Storage = FlaggedStorage<Self>;
}

pub enum Building {
    Mine {
        efficiency: f32,
        input_type: i8,
        output_type: i8,
        capacity: u32,
        capacity_max: u32,
    },
    AutomatedMine {
        efficiency: f32,
        input_type: i8,
        output_type: i8,
    },
    Housing {
        capacity: u32,
        capacity_max: u32,
        quality: i8,
    },
}

impl Component for Building {
    type Storage = DenseVecStorage<Self>;
}

pub struct PlanetsSystem;

impl<'a> System<'a> for PlanetsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Planet>);

    fn run(&mut self, (entities, mut planet): Self::SystemData) {
        for (e, mut planet) in (&*entities, &mut planet).join() {
            println!("Playing with a planet!")
        }
    }
}

pub struct PlanetaryGrowth;

impl<'a> System<'a> for PlanetaryGrowth {
    type SystemData = (Entities<'a>, WriteStorage<'a, Population>);

    fn run(&mut self, (entities, mut population): Self::SystemData) {
        for (e, mut population) in (&*entities, &mut population).join() {
            println!("{}", population.count);
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
            println!("{}", atmosphere.pressure);
            println!("{}", temperature.value);
        }
    }
}

pub struct PlanetaryResource;

impl<'a> System<'a> for PlanetaryResource {
    type SystemData = (Entities<'a>, WriteStorage<'a, Resource>);

    fn run(&mut self, (entities, mut resource): Self::SystemData) {
        for (e, mut resource) in (&*entities, &mut resource).join() {
            println!(
                "{}{}{}",
                resource.resource_type, resource.amount, resource.difficulty
            );
        }
    }
}

pub struct PlanetaryBuilding;

impl<'a> System<'a> for PlanetaryBuilding {
    type SystemData = (Entities<'a>, WriteStorage<'a, Building>);

    fn run(&mut self, (entities, mut building): Self::SystemData) {
        for (e, mut building) in (&*entities, &mut building).join() {
            println!("In building");
        }
    }
}

pub fn initialize_planet(world: &mut World) {
    world
        .create_entity()
        .with(Planet)
        .with(Population { count: 1000000 })
        .with(Atmosphere {
            consistency: Consistency {
                oxygen: 18.0,
                nitrogen: 82.0,
            },
            pressure: 55.5,
        })
        .with(Resource {
            resource_type: 1,
            amount: 300000,
            difficulty: 7,
        })
        .with(Building::Mine {
            efficiency: 100.00,
            input_type: 1,
            output_type: 2,
            capacity: 0,
            capacity_max: 100,
        })
        .with(Building::AutomatedMine {
            efficiency: 100.00,
            input_type: 1,
            output_type: 2,
        })
        .with(Building::Housing {
            capacity: 0,
            capacity_max: 100,
            quality: 50,
        })
        .build();
}
