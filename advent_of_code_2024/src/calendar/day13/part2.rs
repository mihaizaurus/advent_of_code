use advent_of_code_common::types::Machine;
use advent_of_code_common::types::{Direction, Position};

pub fn result(machines: &mut Vec<Machine>) -> isize {
    let mut score = 0;
    
    for machine in machines.iter_mut() {
        machine.update_prize_location();
        score += get_score(machine);
    }
    
    score
}

fn get_score(machine: &Machine) -> isize {
    let mut score = 0;
    let moves = match get_lowest_moves(machine) {
        Some((a,b)) => (a,b),
        None => (0,0)
    };

    if moves != (0,0) {
        score = 3 * moves.0 + moves.1;
    }
    println!("{:?}, moves: {:?}, score: {}", machine, &moves, score);

    score as isize
}

fn get_lowest_moves(machine: &Machine) -> Option<(usize, usize)> {

    let (Direction(ax,ay), Direction(bx, by)) = machine.get_buttons();
    let Position(px, py) = machine.get_prize_location();

    let (gcd_x,_,_) = get_gcd(ax, bx);
    let (gcd_y,_,_) = get_gcd(ay, by);

    println!("GCD X: {}, GCD Y: {}", gcd_x, gcd_y);

    if px % gcd_x as usize != 0 && py % gcd_y as usize != 0 {
        return None
    }

    // a * x1 + b * x2 = px 
    // a * y1 + b * y2 = py
    // solve for a
    // a = (py * x2 - px * y2) / (x1 * y2 - x2 * y1)
    let a = ((py as isize * bx - px as isize * by) / (bx * ay - ax * by)).abs();
    let b = (px as isize - ax * a) / bx;

    println!("a: {}, b: {}", a, b);

    if a * ax + b * bx == px as isize && a * ay + b * by == py as isize {
        println!("I come here");
        return Some((a as usize, b as usize))
    }
    else {
        return None
    }

    
}

fn get_moves_from(v1: isize, v2: isize, v_target: usize, is_b: bool) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    let mut remains = v_target % v1 as usize ;
    if remains >= v_target {
        return vec![];
    }
    while remains < v_target {
        if remains % v2 as usize == 0 {
            let a_moves = (v_target - remains) / v1 as usize;
            let b_moves = (v_target - (v1 as usize * a_moves)) / v2 as usize;
            if is_b {
                moves.push((b_moves, a_moves));
            } else {
                moves.push((a_moves, b_moves));
            }            
            remains += v1 as usize;
        } else {
            remains += v1 as usize;
        }
    } 

    moves
}

fn get_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = get_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn find_positive_bezout(v1: isize, v2: isize, target: isize) -> Option<(usize, usize)> {
    // Compute GCD and base coefficients
    let (g, mut a0, mut b0) = get_gcd(v1, v2);
    
    // Check if target is reachable
    if target % g != 0 {
        return None;
    }

    // Scale coefficients
    let scale = target / g;
    a0 = a0 * scale;
    b0 = b0 * scale;

    // Periodicity
    let period_a = v2 / g;
    let period_b = -v1 / g;

    // Adjust k for positivity
    let k_a = if a0 < 0 {
        (-a0 + period_a - 1) / period_a // Ceil for integer division
    } else {
        0
    };

    let k_b = if b0 < 0 {
        (-b0 + period_b - 1) / period_b // Ceil for integer division
    } else {
        0
    };

    let k = k_a.max(k_b);

    // Compute positive coefficients
    let a = a0 + k * period_a;
    let b = b0 + k * period_b;

    if a >= 0 && b >= 0 {
        Some((a as usize, b as usize))
    } else {
        None
    }

}