use crate::common::types::Machine;
pub fn result(machines: &Vec<Machine>) -> isize {
    let mut score = 0;
    
    for machine in machines {
        score += get_score(machine);
    }
    
    score
}

fn get_score(machine: &Machine) -> isize {
    // get lowest moves of button_a and button_b to reach prize_location
    // button a costs 3 tokens per move
    // button b costs 1 token per move
    // return the lowest cost

    0
}

fn get_lowest_moves(machine: &Machine) -> isize {
    let mut moves = 0;

    moves
}