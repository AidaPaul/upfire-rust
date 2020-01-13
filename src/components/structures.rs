extern crate amethyst;

use crate::components::planets::Deposit;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Manned {
    pub efficiency: u64,
    pub input_type: i8,
    pub output_type: i8,
    pub capacity: u64,
    pub capacity_max: u64,
}

impl Component for Manned {
    type Storage = DenseVecStorage<Self>;
}

impl Manned {
    pub fn mine(&mut self, mut deposit: &mut Deposit) {
        info!("mine: {:?} mining: {:?}", self, deposit);
        let diff_amount;
        if deposit.amount >= self.efficiency {
            diff_amount = deposit.amount - self.efficiency;
            deposit.amount -= self.efficiency;
        } else {
            diff_amount = deposit.amount;
            deposit.amount = 0;
        }
        self.capacity += diff_amount;
    }
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
