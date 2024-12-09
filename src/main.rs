use std::io;
mod day1;
mod day2;

fn main() -> io::Result<()> {
    println!("Hello, advent of code!");
    
    // day1::result()?;
    day2::result()?;
    
    Ok(())
}