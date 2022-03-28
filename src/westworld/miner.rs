use crate::{state, state::State};

#[derive(PartialEq, Eq)]
pub enum Location {
    GoldMine,
}

pub struct Miner<'a> {
    id: usize,
    pub state: &'a dyn State,
    previous_state: &'a dyn State,
    pub location: Location,
    pub gold_carried: usize,
    pub gold_in_bank: usize,
    pub thirst: usize,
    pub fatigue: usize,
}

impl<'a> Miner<'a> {
    fn new(id: usize) -> Self {
        Miner {
            id,
            state: &state::ENTER_MINE_AND_DIG_FOR_NUGGET,
            previous_state: &state::ENTER_MINE_AND_DIG_FOR_NUGGET,
            location: Location::GoldMine,
            gold_carried: 0,
            gold_in_bank: 0,
            thirst: 0,
            fatigue: 0,
        }
    }

    fn update(&mut self) {
        self.thirst += 1;
        self.state.execute(self);
    }

    pub fn change_state(&mut self, new_state: &'a dyn State) {
        self.state.exit(self);
        self.previous_state = self.state;
        self.state = new_state;
        self.state.enter(self);
    }

    pub fn revert_state(&mut self) {
        self.state = self.previous_state;
    }

    pub fn pockets_full(&self) -> bool {
        true
    }

    pub fn thirsty(&self) -> bool {
        true
    }
}
