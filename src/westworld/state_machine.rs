use crate::miner::{Miner, MinerComponents};
use crate::{state, state::State};

pub struct StateMachine {
    current_state: State,
    previous_state: Option<State>,
    global_state: Option<State>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: state::ENTER_MINE_AND_DIG_FOR_NUGGET,
            previous_state: None,
            global_state: None,
        }
    }

    pub fn update(&mut self, owner: &mut MinerComponents) {
        if let Some(global) = self.global_state {
            global.execute(self, owner);
        }

        let current_state = self.current_state;
        current_state.execute(self, owner);
    }

    pub fn change_state(&mut self, owner: &mut MinerComponents, new_state: State) {
        self.previous_state = Some(self.current_state);
        let current_state = self.current_state;
        current_state.exit(self, owner);
        self.current_state = new_state;
        new_state.enter(self, owner);
    }

    fn revert_to_previous_state(&mut self, owner: &mut MinerComponents) {
        if let Some(ps) = self.previous_state {
            self.change_state(owner, ps);
        }
    }
}
