use advent_of_code_common::types::{Position};

pub fn result(tachyon_manifold_grid: &mut Vec<String>) -> isize {
    let start = get_starting_position(tachyon_manifold_grid);
    
    let timelines = list_timelines(tachyon_manifold_grid, start);

    timelines.len() as isize
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

fn list_timelines(tachyon_manifold_grid: &Vec<String>, start: Position) -> Vec<Vec<(Position,char)>>{
    let timelines: Vec<Vec<(Position,char)>> = Vec::new();

    // Traverse all possible splits one by one and count them.

    timelines
}