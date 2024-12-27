use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turnstate: &mut TurnState) {
    let new_state = match turnstate {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turnstate = new_state;
}
