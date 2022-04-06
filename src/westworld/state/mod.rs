use crate::miner::{Location, Miner};
mod miner_states;

pub static ENTER_MINE_AND_DIG_FOR_NUGGET: State = State::EnterMineAndDigForNugget(Emadfn {});
pub static VISIT_BANK_AND_DEPOSIT_GOLD: State = State::VisitBankAndDepositGold(Vbadg {});
pub static QUENCH_THIRST: State = State::QuenchThirst(Qt {});

#[derive(Clone)]
pub enum State {
    EnterMineAndDigForNugget(Emadfn),
    VisitBankAndDepositGold(Vbadg),
    QuenchThirst(Qt),
}

impl State {
    pub fn enter(&self, miner: &mut Miner) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.enter(miner),
            Self::VisitBankAndDepositGold(v) => v.enter(miner),
            Self::QuenchThirst(q) => q.enter(miner),
        }
    }

    pub fn execute(&self, miner: &mut Miner) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.execute(miner),
            Self::VisitBankAndDepositGold(v) => v.execute(miner),
            Self::QuenchThirst(q) => q.execute(miner),
        }
    }
    pub fn exit(&self, miner: &mut Miner) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.exit(miner),
            Self::VisitBankAndDepositGold(v) => v.exit(miner),
            Self::QuenchThirst(q) => q.exit(miner),
        }
    }
}

#[derive(Clone)]
pub struct Emadfn {}
impl Emadfn {
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

#[derive(Clone)]
pub struct Vbadg {}
impl Vbadg {
    fn enter(&self, miner: &mut Miner) {}
    fn execute(&self, miner: &mut Miner) {}
    fn exit(&self, miner: &mut Miner) {}
}

#[derive(Clone, Copy)]
pub struct Qt {}

impl Qt {
    fn enter(&self, miner: &mut Miner) {}
    fn execute(&self, miner: &mut Miner) {}
    fn exit(&self, miner: &mut Miner) {}
}
