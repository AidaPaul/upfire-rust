extern crate amethyst;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use amethyst::{
    ecs::{Entities, Join, NullStorage, ReadStorage, System, World, WriteStorage},
    prelude::*,
};

#[derive(Debug)]
pub struct Mine {
    pub efficiency: f64,
    pub input_type: i8,
    pub output_type: i8,
    pub capacity: u32,
    pub capacity_max: u32,
}

impl Component for Mine {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct AutomatedMine {
    pub efficiency: f64,
    pub input_type: i8,
    pub output_type: i8,
}

impl Component for AutomatedMine {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Housing {
    pub capacity: u32,
    pub capacity_max: u32,
    pub quality: i8,
}

impl Component for Housing {
    type Storage = DenseVecStorage<Self>;
}
