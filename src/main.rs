use std::io;
mod day1;

fn main() -> io::Result<()> {
    println!("Hello, advent of code!");
    
    day1::result()?;
    Ok(())
}