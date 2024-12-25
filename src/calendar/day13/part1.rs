use crate::common::types::Machine;
use crate::common::types::{Direction, Position};

pub fn result(machines: &Vec<Machine>) -> isize {
    let mut score = 0;
    
    for machine in machines {
        score += get_score(machine);
    }
    
    score
}

fn get_score(machine: &Machine) -> isize {
    let score = match get_lowest_moves(machine) {
        Some((a,b)) => 3*a as isize + b as isize,
        None => 0
    };

    println!("{:?}, score: {}", machine, score);

    score
}

fn get_lowest_moves(machine: &Machine) -> Option<(usize, usize)> {

    let ((ax,ay), (bx, by)) = machine.get_buttons();
    let (px, py) = machine.get_prize_location();

    let gcd_x = get_gcd(ax as usize, bx as usize);
    let gcd_y = get_gcd(ay as usize, by as usize);

    println!("GCD X: {}, GCD Y: {}", gcd_x, gcd_y);

    if px % gcd_x == 0 && py % gcd_y == 0 {
        let mut a_moves = 0;
        let mut b_moves = 0;
        if ax > bx {
            let mut remains = px % ax as usize ;
            let mut correct = false;
            if remains >= px {
                return None;
            }
            while correct == false && remains < px {
                if remains % bx as usize == 0 {
                    correct = true;
                } else {
                    remains += ax as usize;
                }
            }
            a_moves = (px - remains) / ax as usize;
            b_moves = (px - (ax as usize * a_moves)) / bx as usize;
        }
        else if bx > ax {
            let mut remains = px % bx as usize ;
            let mut correct = false;
            if remains >= px {
                return None;
            }
            while correct == false {
                if remains % ax as usize == 0 {
                    correct = true;
                } else {
                    remains += bx as usize;
                }
            }
            b_moves = (px - remains) / bx as usize;
            a_moves = (px - (bx as usize * b_moves)) / ax as usize;
        }    
        return Some((a_moves, b_moves));
    }
    else {
        return None;
    }
}

fn check_x_then_y(button_a: Direction, button_b: Direction, prize_location: Position) -> Option<(usize, usize)> {
    let (ax, _) = button_a;
    let (bx, _) = button_b;
    let (px, _) = prize_location;
    let mut a_moves = 0;
        let mut b_moves = 0;
        if ax > bx {
            let mut remains = px % ax as usize ;
            let mut correct = false;
            if remains >= px {
                return None;
            }
            while correct == false && remains < px {
                if remains % bx as usize == 0 {
                    correct = true;
                } else {
                    remains += ax as usize;
                }
            }
            a_moves = (px - remains) / ax as usize;
            b_moves = (px - (ax as usize * a_moves)) / bx as usize;
        }
        else if bx > ax {
            let mut remains = px % bx as usize ;
            let mut correct = false;
            if remains >= px {
                return None;
            }
            while correct == false {
                if remains % ax as usize == 0 {
                    correct = true;
                } else {
                    remains += bx as usize;
                }
            }
            b_moves = (px - remains) / bx as usize;
            a_moves = (px - (bx as usize * b_moves)) / ax as usize;
        }    
        return Some((a_moves, b_moves));
}

fn check_y_then_x(button_a: Direction, button_b: Direction, prize_location: Position) -> (usize, usize) {
    let (_, ay) = button_a;
    let (_, by) = button_b;
    let (_, py) = prize_location;
    let mut a_moves = 0;
        let mut b_moves = 0;
        if ay > by {
            let mut remains = py % ay as usize ;
            let mut correct = false;
            while correct == false {
                if remains % by as usize == 0 {
                    correct = true;
                } else {
                    remains += ay as usize;
                }
            }
            a_moves = (py - remains) / ay as usize;
            b_moves = (py - (ay as usize * a_moves)) / by as usize;
        }
        else if by > ay {
            let mut remains = py % by as usize ;
            let mut correct = false;
            while correct == false {
                if remains % ay as usize == 0 {
                    correct = true;
                } else {
                    remains += by as usize;
                }
            }
            b_moves = (py - remains) / by as usize;
            a_moves = (py - (by as usize * b_moves)) / ay as usize;
        }    
        return (a_moves, b_moves);
}

fn get_gcd(a: usize, b: usize) -> usize {
    let mut gcd = 0;
    for i in (0..a).rev() {
        if a % i == 0 && b % i == 0 {
            gcd = i;
            break;
        }
    }
    gcd
}