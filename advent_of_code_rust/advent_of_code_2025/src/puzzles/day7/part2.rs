use advent_of_code_common::{types::Position};

// Better than the alternative because it doesn't enumerate paths, just counts based on cells of the grid how many have passed there.
// Lightning fast too!

pub fn result(tachyon_manifold_grid: &Vec<String>) -> isize {
    let tmg_bytes = grid_as_bytes(tachyon_manifold_grid);
    let start = get_starting_position(&tmg_bytes);
    let mut ways = ways(&tmg_bytes, start);
    let count_timelines = count_timelines(&tmg_bytes, start, &mut ways);

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

fn count_timelines(tachyon_manifold_grid: &Vec<Vec<u8>>, start: Position, ways: &mut Vec<Vec<u128>>) -> u128 {
    let height = tachyon_manifold_grid.len();
    let width = tachyon_manifold_grid[0].len();
    let Position(_,start_row) = start;
    
    for row in start_row..height-1 {
        for column in 0..width {
            let count = ways[row][column];
            if tachyon_manifold_grid[row+1][column] == b'.' { //continue on
                ways[row+1][column] += count;
            }
            else if tachyon_manifold_grid[row+1][column] == b'^' { //split
                ways[row+1][column-1] += count;
                ways[row+1][column+1] += count;
            }
        }
    }

    count_ways(ways)
}

fn grid_as_bytes(tachyon_manifold_grid: &Vec<String>) -> Vec<Vec<u8>> {
    let mut output_as_bytes: Vec<Vec<u8>> = Vec::new();
    for row in tachyon_manifold_grid {
        let bytes_row = row.as_bytes().to_vec();
        output_as_bytes.push(bytes_row);
    }
    output_as_bytes
}

fn ways(tachyon_manifold_grid: &Vec<Vec<u8>>, start: Position) -> Vec<Vec<u128>> {
    let height = tachyon_manifold_grid.len();
    let width = tachyon_manifold_grid[0].len();
    let Position(start_col,start_row) = start;

    let row: Vec<u128> = vec![0;width];
    let mut grid: Vec<Vec<u128>> = vec![row;height];
    grid[start_row][start_col] = 1; // Start as 1
    grid
}

fn count_ways(ways: &Vec<Vec<u128>>) -> u128 {
    let mut count: u128 = 0;
    let row = ways.len()-1; // last row
    let width = ways[0].len(); 

    for column in 0..width {
        count += ways[row][column];
    }
    
    count
}