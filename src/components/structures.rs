extern crate amethyst;

use crate::components::planets::Deposit;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Manned {
    pub efficiency: f32,
    pub input_type: i8,
    pub output_type: i8,
    pub capacity: f32,
    pub capacity_max: f32,
}

impl Component for Manned {
    type Storage = DenseVecStorage<Self>;
}

impl Manned {
    pub fn mine(&mut self, mut deposit: &mut Deposit, time_delta: f32) {
        let diff_amount;
        let efficiency = self.efficiency * time_delta;
        info!(
            "mine: {:?} mining: {:?} actual eff: {}",
            self, deposit, efficiency
        );
        if deposit.amount >= efficiency {
            diff_amount = deposit.amount - efficiency;
            deposit.amount -= efficiency;
        } else {
            diff_amount = deposit.amount;
            deposit.amount = 0.0;
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
