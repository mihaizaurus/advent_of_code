use std::io;
use std::collections::HashSet;
use advent_of_code_common::types::{Position, Direction, PuzzleAnswer, SimpleGrid};
use advent_of_code_common::utils::{get_input_as_grid,write_result};

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let antenna_grid = AntennaGrid::new(&input_path)?;

    let mut antinode_positions: HashSet<Position> = HashSet::new();
    let mut total_antinode_positions: HashSet<Position> = HashSet::new();
    let frequencies = antenna_grid.get_unique_frequencies();
    for frequency in frequencies {
        let frequency_positions: Vec<Position> = antenna_grid.get_frequency_positions(frequency);
        let antinodes: Vec<Position> = antenna_grid.get_antinodes_positions(&frequency_positions);
        let total_antinodes: Vec<Position> = antenna_grid.get_total_antinodes_positions(&frequency_positions);
        for antinode in antinodes {
            antinode_positions.insert(antinode);
        }
        for antinode in total_antinodes {
            total_antinode_positions.insert(antinode);
        }
        for antenna in frequency_positions {
            total_antinode_positions.insert(antenna);
        }
    }

    let mut output_grid = draw_antinodes(antenna_grid.grid, &total_antinode_positions);

    let part1 = antinode_positions.len() as isize;
    let part2 = total_antinode_positions.len() as isize;
    let puzzle: PuzzleAnswer = (part1, part2);
      
    let pre_last_line = format!("{} unique antinode locations found ",part1);
    let last_line = format!("{} total unique antinode locations found ",part2);
    output_grid.push(pre_last_line);
    output_grid.push(last_line);
    
    write_result(&output_path, &output_grid)?;

    Ok(puzzle)
}

struct AntennaGrid {
    grid: SimpleGrid
}

impl AntennaGrid {
    fn new(input_path: &str) -> io::Result<AntennaGrid>{
        let grid_input = match get_input_as_grid(input_path) {
            Ok(grid_input) => grid_input,
            Err(error) => {
                eprintln!("The Grid could not be built from the input. {}",error);
                return Err(error);
            }
        };
        let antenna_grid = AntennaGrid {
            grid: grid_input
        };
        Ok(antenna_grid)
    }

    fn get_unique_frequencies(&self) -> HashSet<char> {
        let mut frequencies = HashSet::new();
    
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let char = self.grid[row].chars().nth(col).unwrap();
                if char != '.' {
                    frequencies.insert(char);
                }
            }
        }
        return frequencies
    }

    fn get_frequency_positions(&self, frequency: char) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let char = self.grid[row].chars().nth(col).unwrap();
                if char == frequency {
                    positions.push(Position(col, row));
                }
            }
        }
        return positions;
    }

    fn get_antinodes_positions(&self, frequency_positions: &Vec<Position>) -> Vec<Position> {
        let mut antinodes = Vec::new();

        for i in 0..frequency_positions.len() {
            for j in (i + 1)..frequency_positions.len() {
                let frq1 = frequency_positions[i];
                let frq2 = frequency_positions[j];
                let delta = get_difference(frq1, frq2);

                let frq3 = move_to(frq1, '+', delta);
                let frq4 = move_to(frq2, '-', delta);

                for frq in [frq3,frq4] {
                    if frq.1 < self.grid.len() {
                        if frq.0 < self.grid[0].len() {
                            antinodes.push(frq);
                        }
                    }
                }
            }
        }
        return antinodes
    }

    fn get_total_antinodes_positions(&self, frequency_positions: &Vec<Position>) -> Vec<Position> {
        let mut antinodes = Vec::new();

        for i in 0..frequency_positions.len() {
            for j in (i + 1)..frequency_positions.len() {
                let frq1 = frequency_positions[i];
                let frq2 = frequency_positions[j];
                let delta = get_difference(frq1, frq2);
                
                let mut frq3 = move_to(frq1, '+', delta);
                let mut frq4 = move_to(frq2, '-', delta);

                loop {
                    let mut valid = false;

                    if frq3.1 < self.grid.len() 
                        && frq3.0 < self.grid[0].len() 
                        {
                            antinodes.push(frq3);
                            valid = true;
                        } 
                    if frq4.1 < self.grid.len() 
                        && frq4.0 < self.grid[0].len() 
                        {
                            antinodes.push(frq4);
                            valid = true;
                        }
                    if !valid {
                        break;
                    } 

                    frq3 = move_to(frq3, '+', delta);
                    frq4 = move_to(frq4, '-', delta);
                }
                
            }
        }

        return antinodes
    }

}

fn move_to(position: Position, sign: char, direction: Direction) -> Position {
    match sign {
        '+' => Position((position.0 as isize + direction.0) as usize , (position.1 as isize + direction.1) as usize),
        '-' => Position((position.0 as isize - direction.0) as usize , (position.1 as isize - direction.1) as usize),
        _ => Position(0,0)
    }
}

fn get_difference(pos1: Position, pos2:Position) -> Direction {
    Direction(pos1.0 as isize - pos2.0 as isize , pos1.1 as isize - pos2.1 as isize)
}

fn draw_antinodes(grid: SimpleGrid, antinode_positions: &HashSet<Position>) -> SimpleGrid {
    let mut drawn_grid: SimpleGrid = grid.clone();

    for Position(col, row) in antinode_positions {
        let col_index = *col as usize;
        let row_index = *row as usize;

        if let Some(row_string) = drawn_grid.get_mut(row_index) {
            let mut chars: Vec<char> = row_string.chars().collect();

            if let Some(ch) = chars.get_mut(col_index) {
                if *ch == '.' {
                    *ch = '#';
                }
            }
            *row_string = chars.into_iter().collect();
        }
    }

    return drawn_grid;
}