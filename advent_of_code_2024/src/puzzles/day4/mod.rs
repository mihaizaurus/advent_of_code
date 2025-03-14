use std::fs::File;
use std::io::{self,BufRead, Write};

pub fn result(input_path: &str, output_path: &str) -> io::Result<(isize, isize)> {

    const WORD: &str = "XMAS";
    
    let input_string = get_input_text(input_path)?;
    let count_xmas = find_instances_of_word(&input_string, WORD)?;

    let count_x_mas = find_instances_of_cross(&input_string)?;

    let mut file = File::create(output_path)?;
    
    writeln!(file, "number of occurrences of '{}' found = {}",WORD,count_xmas)?;
    writeln!(file, "number of occurrences of 'X-MAS' found = {}",count_x_mas)?;

    Ok((count_xmas as isize, count_x_mas as isize))
}

fn get_input_text(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut input_text = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input_text.push(line);
    }

    Ok(input_text)
}

fn find_instances_of_word(input_string: &Vec<String>, word_to_match: &str) -> io::Result<usize> {
    let mut xmas_list = Vec::new();
    
    let word_chars: Vec<char> = word_to_match.chars().collect();

    for y in 0..input_string.len() {
        for x in 0..input_string[y].len() {
            if input_string[y].chars().nth(x).unwrap() == word_chars[0] {
                for (dx, dy) in DIRECTIONS {
                    if can_find_word(&input_string, word_chars.as_slice(), x, y, dx, dy) {
                        xmas_list.push((x,y,(dx,dy))); 
                        // println!("{:?} found at position ({x},{y}) and pointing in direction ({dx},{dy})",word_to_match);
                    }
                }
            }
        }
    }

    Ok(xmas_list.len())
}

fn find_instances_of_cross(input_string: &Vec<String>) -> io::Result<usize> {
    let mut updated_xmas_list = Vec::new();
    
    let word= "MAS";
    let word_chars : Vec<char> = word.chars().collect();

    for y in 1..(input_string.len() -1 ) {
        for x in 1..(input_string[y].len() -1 ) {
            if input_string[y].chars().nth(x).unwrap() == word_chars[1] { // test for middle character 'A' , not first...
                if can_find_pattern(&input_string, word_chars.as_slice(), x, y) {
                    updated_xmas_list.push((x,y));
                }
            }
        }
    }

    Ok(updated_xmas_list.len())
}

fn can_find_pattern(grid: &Vec<String>, word: &[char], x: usize, y: usize) -> bool {
    for (dx, dy) in DIAGONALS {

        let first_mas = grid[(y as isize + dy) as usize].chars().nth((x as isize + dx) as usize).unwrap() == word[0] && grid[(y as isize - dy) as usize].chars().nth((x as isize - dx) as usize).unwrap() == word[2];
        let second_mas_option_1 = grid[(y as isize + dy) as usize].chars().nth((x as isize - dx) as usize).unwrap() == word[0] && grid[(y as isize - dy) as usize].chars().nth((x as isize + dx) as usize).unwrap() == word[2];
        let second_mas_option_2 = grid[(y as isize - dy) as usize].chars().nth((x as isize + dx) as usize).unwrap() == word[0] && grid[(y as isize + dy) as usize].chars().nth((x as isize - dx) as usize).unwrap() == word[2];

        if first_mas && second_mas_option_1 || first_mas && second_mas_option_2 {
            return true
        }
    }
    return false
}

fn can_find_word(grid: &Vec<String>, word: &[char], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let mut cx = x as isize;
    let mut cy = y as isize;
    
    for &ch in word {
        if cx < 0 || cy < 0 || cy >= grid.len() as isize || cx >= grid[cy as usize].len() as isize {
            return false // Out of bounds
        }

        if grid[cy as usize].chars().nth(cx as usize).unwrap() != ch {
            return false // Does not match word
        }
        
        cx += dx;
        cy += dy;   
    }
    return true
}

const DIRECTIONS: [(isize, isize);8] = [
    DIAGONALS[0],    // Up-Left Diagonal
    (0,-1),     // Up
    DIAGONALS[1],     // Up-Right Diagonal
    (1,0),      // Right
    DIAGONALS[2],      // Down-Right Diagonal
    (0,1),      // Down
    DIAGONALS[3],     // Down-Left Diagonal
    (-1,0),     // Left
];

const DIAGONALS: [(isize, isize);4] = [
    (-1,-1),    // Up-Left Diagonal
    (1,-1),     // Up-Right Diagonal
    (1,1),      // Down-Right Diagonal
    (-1,1),     // Down-Left Diagonal
];