extern crate amethyst;

use crate::components::structures::Manned;
use amethyst::ecs::{Component, FlaggedStorage, NullStorage, VecStorage};

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
    pub amount: u64,
    pub difficulty: i8,
}

#[derive(Default, Debug)]
pub struct Deposits {
    pub natural: Vec<Deposit>,
}

impl Component for Deposits {
    type Storage = VecStorage<Self>;
}

#[derive(Default, Debug)]
pub struct Mines {
    pub manned: Vec<Manned>,
}

impl Component for Mines {
    type Storage = VecStorage<Self>;
}
