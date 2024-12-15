use std::io;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> io::Result<()> {
    println!("Hello, advent of code!");
    
    day1::result()?; // Passed
    day2::result()?; // Passed
    day3::result()?; // Passed
    day4::result()?; // Passed
    day5::result()?; // Passed
    day6::result()?;

    Ok(())
}