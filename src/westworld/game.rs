use crate::state::StateMachine;
use crate::miner::Miner;

pub struct Game{
    state_machine:StateMachine,
    entities: Vec<Miner>
}

impl Game{
  pub fn update(&self){
    println!("starting");
    for ent in entities{
        ent.update();
    }
  }
