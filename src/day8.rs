use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn result(input_path: &str, output_path: &str) -> io::Result<(isize, isize)> {
    // let inputs = get_input(input_path)?;
    let test = 14;
    
    write_result(&output_path, "Day 8")?;

    Ok((test as isize, test as isize))
}

fn write_result(output_path: &str, something: &str) -> io::Result<()> {
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    writeln!(file,"This is a placeholder for {}", something)?;

    Ok(())
}