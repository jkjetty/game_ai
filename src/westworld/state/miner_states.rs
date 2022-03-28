use super::{Location, Miner, State};

pub static ENTER_MINE_AND_DIG_FOR_NUGGET: EnterMineAndDigForNugget = EnterMineAndDigForNugget {};
pub static VISIT_BANK_AND_DEPOSIT_GOLD: VisitBankAndDepositGold = VisitBankAndDepositGold {};
pub static QUENCH_THIRST: QuenchThirst = QuenchThirst {};

pub struct EnterMineAndDigForNugget {}

impl State for EnterMineAndDigForNugget {
    fn enter(&self, miner: &mut Miner) {
        if miner.location != Location::GoldMine {
            println!("walking to the gold mine");
            miner.location = Location::GoldMine;
        }
    }

    fn execute(&self, miner: &mut Miner) {
        miner.gold_carried += 1;
        miner.fatigue += 1;
        println!("picking up a nugget");
        if miner.pockets_full() {
            miner.change_state(&VISIT_BANK_AND_DEPOSIT_GOLD);
        }

        if miner.thirsty() {
            miner.change_state(&QUENCH_THIRST);
        }
    }
    fn exit(&self, miner: &mut Miner) {
        println!("leaving the gold mine with lots of gold!");
    }
}

pub struct VisitBankAndDepositGold {}

impl State for VisitBankAndDepositGold {
    fn enter(&self, miner: &mut Miner) {}
    fn execute(&self, miner: &mut Miner) {}
    fn exit(&self, miner: &mut Miner) {}
}

pub struct QuenchThirst {}

impl State for QuenchThirst {
    fn enter(&self, miner: &mut Miner) {}
    fn execute(&self, miner: &mut Miner) {}
    fn exit(&self, miner: &mut Miner) {}
}
