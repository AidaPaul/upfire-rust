extern crate amethyst;
use amethyst::ecs::NullStorage;
use amethyst::ecs::{Component, Entity, FlaggedStorage, VecStorage};
use amethyst::prelude::*;

pub fn generate_planet(label: String) -> Planet {
    Planet {
        label,
        atmosphere: Default::default(),
        deposits: vec![],
        mines: vec![],
    }
}

#[derive(Default, Debug)]
pub struct Planet {
    pub label: String,
    pub atmosphere: Atmosphere,
    pub deposits: Vec<Entity>,
    pub mines: Vec<Entity>,
}

impl Component for Planet {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Debug)]
pub struct MineableTag;

impl Component for MineableTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Population {
    pub count: u32,
}

impl Component for Population {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Temperature {
    pub value: f64,
}

impl Component for Temperature {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Consistency {
    pub oxygen: f64,
    pub nitrogen: f64,
    pub co2: f64,
}

#[derive(Default, Debug)]
pub struct Atmosphere {
    pub consistency: Consistency,
    pub pressure: f64,
}

impl Component for Atmosphere {
    type Storage = VecStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Deposit {
    pub deposit_type: i8,
    pub amount: f64,
    pub difficulty: i8,
}

impl Component for Deposit {
    type Storage = VecStorage<Self>;
}
