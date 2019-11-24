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
    pub count: u32,
}

impl Component for Population {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Temperature {
    pub value: f64,
}

impl Component for Temperature {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Debug)]
pub struct Consistency {
    pub oxygen: f64,
    pub nitrogen: f64,
    pub co2: f64,
}

#[derive(Debug)]
pub struct Atmosphere {
    pub consistency: Consistency,
    pub pressure: f64,
}

impl Component for Atmosphere {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Resource {
    pub resource_type: i8,
    pub amount: f64,
    pub difficulty: i8,
}

impl Component for Resource {
    type Storage = FlaggedStorage<Self>;
}
