pub trait TStateMachine {
    type Owner;
}

pub struct StateMachine<T> {}

impl<T> TStateMachine for StateMachine<T> {
    type Owner = T;
}
