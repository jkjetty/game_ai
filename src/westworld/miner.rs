use crate::{state, state::State, state_machine::StateMachine};

#[derive(PartialEq, Eq)]
pub enum Location {
    GoldMine,
    Bank,
    Home,
    Saloon,
}

pub struct Miner {
    id: usize,
    pub state_machine: StateMachine,
    components: MinerComponents,
}

impl Miner {
    pub fn new(id: usize) -> Self {
        let sm = StateMachine::new();
        Miner {
            id,
            state_machine: sm,
            components: MinerComponents {
                location: Location::GoldMine,
                gold_carried: 0,
                gold_in_bank: 0,
                thirst: 0,
                fatigue: 0,
            },
        }
    }

    pub fn change_state(&mut self, target: State) {
        // need to remove the circular miner -> sm -> miner dependency
        self.state_machine
            .change_state(&mut self.components, target);
    }

    pub fn update(&mut self) {
        self.components.update();
        self.state_machine.update(&mut self.components);
    }
}

pub struct MinerComponents {
    pub location: Location,
    pub gold_carried: usize,
    pub gold_in_bank: usize,
    pub thirst: usize,
    pub fatigue: usize,
}
impl MinerComponents {
    pub fn pockets_full(&self) -> bool {
        true
    }

    pub fn thirsty(&self) -> bool {
        true
    }

    pub fn update(&mut self) {
        self.thirst += 1;
    }

    pub fn put_gold_in_bank(&mut self) {
        self.gold_in_bank += self.gold_carried;
        self.gold_carried = 0;
    }
}
