use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn get_input_as_grid(input_path: &str) -> io::Result<Vec<String>> {
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open file at {}. Details: {}", input_path, error);
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

pub fn write_result(output_path: &str, grid: &Vec<String>) -> io::Result<()> {    
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    for line in grid {
        writeln!(file, "{}",line)?;
    }

    Ok(())
}