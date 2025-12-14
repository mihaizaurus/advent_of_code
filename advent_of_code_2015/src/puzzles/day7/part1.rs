use advent_of_code_common::types::{Position, Direction, DIRECTIONS};
use advent_of_code_common::utils::set;

pub fn result(tachyon_manifold_grid: &mut Vec<String>) -> isize {
    let start = get_starting_position(tachyon_manifold_grid);
    
    traverse_tachyon(tachyon_manifold_grid, start);

    let total_sum = count_split_beams(tachyon_manifold_grid);
    total_sum as isize
}

fn get_starting_position(tachyon_manifold_grid: &Vec<String>) -> Position {
    let (mut x,mut y) = (0,0);
    let height = tachyon_manifold_grid.len();
    let width = tachyon_manifold_grid[0].len();

    for h in 0..height {
        for w in 0..width {
            if tachyon_manifold_grid[h].chars().nth(w).unwrap() == 'S' {
                (x,y) = (w,h);
            }
        }
    }

    Position(x,y)
}

fn traverse_tachyon(tachyon_manifold_grid: &mut Vec<String>, start: Position) {
    let mut tmg = tachyon_manifold_grid;
    let Position(sx, sy): Position = start;
    let height = tmg.len();
    let width = tmg[0].len();

    set(&mut tmg, Position(sx,sy+1), '|');

    for h in 1..height {
        for w in 0..width {
            if tmg[h].chars().nth(w).unwrap() == '^' && tmg[h-1].chars().nth(w).unwrap() == '|' { //beam is split
                if w-1 >= 0 {
                    set(&mut tmg, Position(w-1,h), '|');
                }
                if w+1 <= width {
                    set(&mut tmg, Position(w+1,h), '|'); // Right
                }
            }
            else if tmg[h].chars().nth(w).unwrap() == '.' && tmg[h-1].chars().nth(w).unwrap() == '|' { //beam is coming down
                set(&mut tmg, Position(w,h), '|');
            }
        }
    }
}

fn count_split_beams(tachyon_manifold_grid: &Vec<String>) -> usize {
    let mut count = 0;
    let height = tachyon_manifold_grid.len();
    let width = tachyon_manifold_grid[0].len();

    for h in 0..height {
        for w in 0..width {
            if tachyon_manifold_grid[h].chars().nth(w).unwrap() == '^' && tachyon_manifold_grid[h-1].chars().nth(w).unwrap() == '|' { //beam is split
                count += 1;
            }
        }
    }
    count
}