use crate::miner::{Location, Miner};
mod miner_states;
pub use miner_states::{ENTER_MINE_AND_DIG_FOR_NUGGET, QUENCH_THIRST, VISIT_BANK_AND_DEPOSIT_GOLD};

pub trait State {
    fn enter(&self, miner: &mut Miner);
    fn execute(&self, miner: &mut Miner);
    fn exit(&self, miner: &mut Miner);
}
