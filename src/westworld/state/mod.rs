use crate::miner::{Location, Miner, MinerComponents};
use crate::state_machine::StateMachine;
mod miner_states;

pub static ENTER_MINE_AND_DIG_FOR_NUGGET: State = State::EnterMineAndDigForNugget(Emadfn {});
pub static VISIT_BANK_AND_DEPOSIT_GOLD: State = State::VisitBankAndDepositGold(Vbadg {});
pub static QUENCH_THIRST: State = State::QuenchThirst(Qt {});
pub static GO_HOME_AND_SLEEP_TILL_RESTED: State = State::GoHomeAndSleepTillRested(Ghastr {});

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    EnterMineAndDigForNugget(Emadfn),
    VisitBankAndDepositGold(Vbadg),
    QuenchThirst(Qt),
    GoHomeAndSleepTillRested(Ghastr),
}

impl State {
    pub fn enter(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.enter(state_machine, miner),
            Self::VisitBankAndDepositGold(v) => v.enter(state_machine, miner),
            Self::QuenchThirst(q) => q.enter(state_machine, miner),
            Self::GoHomeAndSleepTillRested(g) => g.enter(state_machine, miner),
        }
    }

    pub fn execute(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.execute(state_machine, miner),
            Self::VisitBankAndDepositGold(v) => v.execute(state_machine, miner),
            Self::QuenchThirst(q) => q.execute(state_machine, miner),
            Self::GoHomeAndSleepTillRested(g) => g.execute(state_machine, miner),
        }
    }
    pub fn exit(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        match self {
            Self::EnterMineAndDigForNugget(e) => e.exit(state_machine, miner),
            Self::VisitBankAndDepositGold(v) => v.exit(state_machine, miner),
            Self::QuenchThirst(q) => q.exit(state_machine, miner),
            Self::GoHomeAndSleepTillRested(g) => g.execute(state_machine, miner),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Emadfn {}
impl Emadfn {
    fn enter(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        if miner.location != Location::GoldMine {
            println!("walking to the gold mine");
            miner.location = Location::GoldMine;
        }
    }

    fn execute(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        miner.gold_carried += 1;
        miner.fatigue += 1;
        println!("picking up a nugget");
        if miner.pockets_full() {
            state_machine.change_state(miner, VISIT_BANK_AND_DEPOSIT_GOLD);
        }

        if miner.thirsty() {
            state_machine.change_state(miner, QUENCH_THIRST);
        }
    }
    fn exit(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        println!("leaving the gold mine with lots of gold!");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vbadg {}
impl Vbadg {
    fn enter(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        if miner.location != Location::Bank {
            println!("goin to the bank");
            miner.location = Location::Bank;
        }
    }

    fn execute(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {
        miner.put_gold_in_bank();
        println!("put gold in bank, now have {}", miner.gold_carried);

        if miner.gold_in_bank > 5 {
            println!("got enough money, heading back home");
            state_machine.change_state(miner, GO_HOME_AND_SLEEP_TILL_RESTED);
        } else {
            state_machine.change_state(miner, ENTER_MINE_AND_DIG_FOR_NUGGET);
        }
    }

    fn exit(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Qt {}

impl Qt {
    fn enter(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
    fn execute(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
    fn exit(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ghastr {}

impl Ghastr {
    fn enter(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
    fn execute(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
    fn exit(&self, state_machine: &mut StateMachine, miner: &mut MinerComponents) {}
}
