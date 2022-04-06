use crate::state::State;
use crate::Miner;

//pub trait TStateMachine {
//    type Owner;
//}

pub struct StateMachine<'a> {
    current_state: State,
    previous_state: State,
    global_state: Option<State>,
    owner: Miner<'a>,
}

//impl<T, R, S, O> TStateMachine for StateMachine<T, R, S, O>
//where
//    T: State,
//    R: State,
//    S: State,
//{
//    type Owner = O;
//}

impl<'a> StateMachine<'a> {
    fn update(&mut self) {
        if let Some(ref global) = self.global_state {
            global.execute(&mut self.owner);
        }

        self.current_state.execute(&mut self.owner);
    }

    fn change_state(&mut self, new_state: State) {
        self.previous_state = self.current_state.clone();
        self.current_state.exit(&mut self.owner);
        self.current_state = new_state;
        self.current_state.enter(&mut self.owner);
    }

    fn revert_to_previous_state(&mut self) {
        self.change_state(self.previous_state.clone());
    }
}
