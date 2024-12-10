use std::io;
mod day1;
mod day2;
mod day3;

fn main() -> io::Result<()> {
    println!("Hello, advent of code!");
    
    // day1::result()?;
    // day2::result()?;
    day3::result()?;

    Ok(())
}