use super::types::{DIRECTIONS, Direction, Position, SimpleGrid};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub fn get_test_input_path(year: usize, day: usize, suffix: Option<&str>) -> PathBuf {
    let suffix = suffix.unwrap_or("");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    let path = path
        .join("..")
        .join("advent_of_code_inputs")
        .join(format!("{}", year))
        .join("tests")
        .join(format!("day{}{}.txt", day, suffix));
    let canonical_path = path.canonicalize().unwrap_or(path.clone());

    // For Windows, strip `\\?\` prefix if present
    if let Some(stripped) = canonical_path.to_str() {
        if stripped.starts_with(r"\\?\") {
            return PathBuf::from(&stripped[4..]);
        }
    }
    canonical_path
}

pub fn get_test_results_path(year: usize, day: usize, suffix: Option<&str>) -> PathBuf {
    let suffix = suffix.unwrap_or("");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("advent_of_code_results")
        .join("rust")
        .join(format!("{}", year))
        .join("tests")
        .join(format!("day{}{}.txt", day, suffix));

    //create folders if missing
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create directory {:?}: {}", parent, e);
        } else {
            // println!("Created or verified directory {:?}", parent);
        }
    }

    let canonical_path = path.canonicalize().unwrap_or(path.clone());

    // For Windows, strip `\\?\` prefix if present
    if let Some(stripped) = canonical_path.to_str() {
        if stripped.starts_with(r"\\?\") {
            return PathBuf::from(&stripped[4..]);
        }
    }

    canonical_path
}

pub fn get_puzzle_input_path(year: usize, day: usize) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("advent_of_code_inputs")
        .join(format!("{}", year))
        .join("puzzles")
        .join(format!("day{}.txt", day));
    let canonical_path = path.canonicalize().unwrap_or(path.clone());

    // For Windows, strip `\\?\` prefix if present
    if let Some(stripped) = canonical_path.to_str() {
        if stripped.starts_with(r"\\?\") {
            return PathBuf::from(&stripped[4..]);
        }
    }

    canonical_path
}

pub fn get_puzzle_results_path(year: usize, day: usize) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("advent_of_code_results")
        .join("rust")
        .join(format!("{}", year))
        .join("puzzles")
        .join(format!("day{}.txt", day));

    //create folders if missing
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create directory {:?}: {}", parent, e);
        } else {
            println!("Created or verified directory {:?}", parent);
        }
    }

    let canonical_path = path.canonicalize().unwrap_or(path.clone());

    // For Windows, strip `\\?\` prefix if present
    if let Some(stripped) = canonical_path.to_str() {
        if stripped.starts_with(r"\\?\") {
            return PathBuf::from(&stripped[4..]);
        }
    }

    canonical_path
}

pub fn get_input_as_grid(input_path: &str) -> io::Result<Vec<String>> {
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "Error: Failed to open file at {}. Details: {}",
                input_path, error
            );
            return Err(error);
        }
    };
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        grid.push(line);
    }
    Ok(grid)
}

pub fn get_input_as_vector(input_path: &str) -> io::Result<Vec<usize>> {
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "Error: Failed to open file at {}. Details: {}",
                input_path, error
            );
            return Err(error);
        }
    };
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let result: Vec<usize> = line
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    Ok(result)
}

pub fn get_input_as_vector_128(input_path: &str) -> io::Result<Vec<u128>> {
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "Error: Failed to open file at {}. Details: {}",
                input_path, error
            );
            return Err(error);
        }
    };
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let result: Vec<u128> = line
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    Ok(result)
}

pub fn write_result(output_path: &str, grid: &Vec<String>) -> io::Result<()> {
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "Error: Unable to create file at path {}. Details: {}",
                output_path, error
            );
            return Err(error);
        }
    };

    for line in grid {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn set(map: &mut Vec<String>, position: Position, c: char) {
    let x = position.0 as usize;
    let y = position.1 as usize;
    if let Some(row) = map.get_mut(y) {
        let mut chars: Vec<char> = row.chars().collect();
        if let Some(ch) = chars.get_mut(x) {
            *ch = c;
        }
        *row = chars.into_iter().collect();
    }
}

pub fn get_char_at(Position(col, row): Position, map: &SimpleGrid) -> String {
    return map[row as usize]
        .chars()
        .nth(col as usize)
        .unwrap()
        .to_string();
}

pub fn get_neighbors(start: Position, neighbor: String, map: &SimpleGrid) -> Vec<Position> {
    let mut neighbors: Vec<Position> = Vec::new();

    for direction in DIRECTIONS {
        if let Some(new_pos) = move_from(start, direction, &map) {
            if get_char_at(Position(new_pos.0 as usize, new_pos.1 as usize), map) == neighbor {
                neighbors.push(new_pos);
            };
        }
    }

    return neighbors;
}

pub fn move_from(position: Position, towards: Direction, map: &SimpleGrid) -> Option<Position> {
    if (position.0 as isize + towards.0) as usize >= map[0].len()
        || (position.1 as isize + towards.1) as usize >= map.len()
        || (position.0 as isize + towards.0) < 0
        || (position.1 as isize + towards.1) < 0
    {
        return None;
    } else {
        return Some(position + towards);
    }
}

fn is_in_bounds(neighbor: Position, map: SimpleGrid) -> bool {
    let Position(row, col) = neighbor;
    row > 0 && row <= map.len() && col > 0 && col <= map[row].len()
}

