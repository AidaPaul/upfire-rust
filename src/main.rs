extern crate amethyst;

use amethyst::{
    core::{
        bundle::SystemBundle,
        frame_limiter::FrameRateLimitStrategy,
        shrev::{EventChannel, ReaderId},
    },
    ecs::{DispatcherBuilder, Entities, Join, Read, ReadStorage, Resources, System, SystemData, World, Write, WriteStorage, NullStorage},
    prelude::*,
};
use amethyst::ecs::{Component, VecStorage, DenseVecStorage, FlaggedStorage};

use amethyst::Error;
use core::result::Result;

#[derive(Debug)]
struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PlanetsSystem, "planets_system", &[]);
        builder.add(PlanetaryGrowth, "planetary_growth_system", &[]);
        builder.add(PlanetaryAtmosphere, "planetary_atmosphere_system", &[]);
        builder.add(PlanetaryResource, "planetary_resource_system", &[]);
        builder.add(PlanetaryBuilding, "planetary_buildings_system", &[]);
        Ok(())
    }
}

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
struct Consistency {
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

enum Building {
    Mine {efficiency: f32, input_type: i8, output_type: i8, capacity: u32, capacity_max: u32},
    AutomatedMine {efficiency: f32, input_type: i8, output_type: i8},
    Housing {capacity: u32, capacity_max: u32, quality: i8},
}

impl Component for Building {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_planet(world: &mut World) {
    world
        .create_entity()
        .with(Planet)
        .with(Population {count: 1000000})
        .with(Atmosphere {consistency: Consistency {oxygen: 18.0, nitrogen: 82.0}, pressure: 55.5})
        .with(Resource {resource_type: 1, amount: 300000, difficulty: 7})
        .with(Building::Mine {efficiency: 100.00, input_type: 1, output_type:2, capacity: 0, capacity_max: 100})
        .with(Building::AutomatedMine {efficiency: 100.00, input_type: 1, output_type:2})
        .with(Building::Housing {capacity: 0, capacity_max: 100, quality: 50})
        .build();
}

struct PlanetsSystem;

impl<'a> System<'a> for PlanetsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Planet>,
    );

    fn run(&mut self, (entities, mut planet): Self::SystemData) {
        for (e, mut planet) in (&*entities, &mut planet).join() {
            println!("Playing with a planet!") 
        }
    }
}

struct PlanetaryGrowth;

impl<'a> System<'a> for PlanetaryGrowth {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Population>,
    );

    fn run(&mut self, (entities, mut population): Self::SystemData) {
        for (e, mut population) in (&*entities, &mut population).join() {
            println!("{}", population.count);
        }
    }
}

struct PlanetaryAtmosphere;

impl<'a> System<'a> for PlanetaryAtmosphere {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Atmosphere>,
        WriteStorage<'a, Temperature>,
    );

    fn run(&mut self, (entities, mut atmosphere, mut temperature): Self::SystemData) {
        for (e, mut atmosphere, mut temperature) in (&*entities, &mut atmosphere, &mut temperature).join() {
            println!("{}", atmosphere.pressure);
            println!("{}", temperature.value);
        }
    }
}

struct PlanetaryResource;

impl<'a> System<'a> for PlanetaryResource {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Resource>,
    );

    fn run(&mut self, (entities, mut resource): Self::SystemData) {
        for (e, mut resource) in (&*entities, &mut resource).join() {
            println!("{}{}{}", resource.resource_type, resource.amount, resource.difficulty);
        }
    }    
}

struct PlanetaryBuilding;

impl<'a> System<'a> for PlanetaryBuilding {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Building>,
    );

    fn run(&mut self, (entities, mut building): Self::SystemData) {
        for (e, mut building) in (&*entities, &mut building).join() {
            println!("In building");
        }
    }    
}

struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData {world, .. } = data;
        initialize_planet(world);
    }
}


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let world = World::new();

    let game_data = GameDataBuilder::default().with_bundle(MyBundle)?;

    let mut game = Application::build("./", GameplayState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 1)
        .build(game_data)?;

    game.run();
    Ok(())
}
