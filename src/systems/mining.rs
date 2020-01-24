extern crate amethyst;

use amethyst::core::Time;
use amethyst::ecs::*;

use crate::components::planets::*;

pub struct MiningSystem;

impl<'a> System<'a> for MiningSystem {
    type SystemData = (
        WriteStorage<'a, Mines>,
        WriteStorage<'a, Deposits>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut mines, mut deposits, time): Self::SystemData) {
        for (mines, deposits) in (&mut mines, &mut deposits).join() {
            for deposit in deposits.natural.iter_mut() {
                if deposit.amount == 0.0 {
                    debug!("Deposit {:?} is empty, skipping", deposit);
                    continue;
                }
                for mine in mines.manned.iter_mut() {
                    if (mine.capacity - mine.capacity_max).abs() < 0.1 {
                        debug!("Mine {:?} is full, skipping", mine);
                        continue;
                    }
                    mine.mine(deposit, time.delta_seconds());
                    if deposit.amount == 0.0 {
                        break;
                    }
                }
            }
        }
    }
}

pub struct EmptyDepositRemovalSystem;

impl<'a> System<'a> for EmptyDepositRemovalSystem {
    type SystemData = WriteStorage<'a, Deposits>;

    fn run(&mut self, mut data: Self::SystemData) {
        for deposits in (&mut data).join() {
            debug!("{:?}", deposits);
            deposits.natural.retain(|x| x.amount > 0.0);
        }
    }
}
