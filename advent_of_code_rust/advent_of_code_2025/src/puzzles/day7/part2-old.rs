use advent_of_code_common::{types::Position};

pub fn result(tachyon_manifold_grid: &Vec<String>) -> isize {
    let tmg_bytes = grid_as_bytes(tachyon_manifold_grid);
    let start = get_starting_position(&tmg_bytes);
    
    let count_timelines = count_timelines(&tmg_bytes, start);

    count_timelines as isize
}

fn get_starting_position(tachyon_manifold_grid: &Vec<Vec<u8>>) -> Position {
    let (mut x,mut y) = (0,0);
    let height = tachyon_manifold_grid.len();
    let width = tachyon_manifold_grid[0].len();

    for h in 0..height {
        for w in 0..width {
            if tachyon_manifold_grid[h][w] == b'S' {
                (x,y) = (w,h);
            }
        }
    }

    Position(x,y)
}

fn count_timelines(tachyon_manifold_grid: &Vec<Vec<u8>>, start: Position) -> usize {
    let mut active_timelines: Vec<Position> = Vec::new();
    let mut completed_timelines = 0;

    active_timelines.push(start);

    while active_timelines.len() > 0 {
        let mut current_timeline = active_timelines.pop().unwrap();
        loop {
            let Position(last_x,last_y) = current_timeline;
            if last_y + 1 == tachyon_manifold_grid.len(){ // path is complete
                completed_timelines += 1;
                break;
            }
            else if tachyon_manifold_grid[last_y+1][last_x] == b'.'{ // straight ahead
                current_timeline = Position(last_x,last_y+1);
                continue;
            }
            else if tachyon_manifold_grid[last_y+1][last_x] == b'^'{ // split
                active_timelines.push(Position(last_x-1,last_y+1));
                active_timelines.push(Position(last_x+1,last_y+1));
                break;
            }
        }        
    }
    
    completed_timelines
}

fn grid_as_bytes(tachyon_manifold_grid: &Vec<String>) -> Vec<Vec<u8>> {
    let mut output_as_bytes: Vec<Vec<u8>> = Vec::new();
    for row in tachyon_manifold_grid {
        let bytes_row = row.as_bytes().to_vec();
        output_as_bytes.push(bytes_row);
    }
    output_as_bytes
}