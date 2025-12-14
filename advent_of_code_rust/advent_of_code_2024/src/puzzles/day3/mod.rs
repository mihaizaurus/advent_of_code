use std::fs::File;
use std::io::{self, Read, Write};
use regex::Regex;

pub fn result(input_path: &str, output_path: &str) -> io::Result<(isize, isize)> {
    
    let input_string = get_inputs(input_path)?;
    let sum_of_mult = sum_of_mult(&input_string)?;

    let split_inputs = get_inputs_split(&input_string)?;
    let corrected_sum = process_correct_inputs(split_inputs)?;

    let mut file = File::create(output_path)?;
    
    writeln!(file, "sum of multipliers = {}",sum_of_mult)?;
    writeln!(file, "corrected sum of 'do()' multipliers = {}",corrected_sum)?;

    Ok((sum_of_mult as isize, corrected_sum as isize))
}

fn get_inputs(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;

    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    Ok(input_string)
}

fn get_inputs_split(input_string: &str) -> io::Result<Vec<&str>> {
    let mut split_inputs = Vec::new();
    let mut start_index = 0;

    //Note that r"(.*?)(do\(\)|don't\(\)).*?(?=(do\(\)|don't\(\))|$)" Won't work as the Regex crate doesn't support lookaround... and r"(do\(\)|don't\(\)).*" overshoots
    let pattern = r"(do\(\)|don't\(\))";
    let re = Regex::new(pattern).unwrap();

    //had to resort to manual split...
    for input in re.find_iter(input_string) {
        let end_index = input.start();
        if start_index < end_index {
            split_inputs.push(&input_string[start_index..end_index]);
        }
        start_index = input.start();
    }

    if start_index < input_string.len() {
        split_inputs.push(&input_string[start_index..]);
    }

    Ok(split_inputs)
}

fn sum_of_mult(input_string: &str) -> io::Result<i32> {
    let mut sum_of_mult = 0;

    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    for (_,[num_1,num_2]) in re.captures_iter(&input_string).map(|c| c.extract()) {
        sum_of_mult += num_1.parse::<i32>().unwrap() * num_2.parse::<i32>().unwrap();
    }

    Ok(sum_of_mult)
}

fn process_correct_inputs(split_inputs: Vec<&str>) -> io::Result<i32> {
    let mut corrected_sum = 0;

    corrected_sum += sum_of_mult(split_inputs[0])?;

    for input in split_inputs.iter().filter(|block| block.starts_with("do()")) {
        corrected_sum += sum_of_mult(input)?
    }

    Ok(corrected_sum)
}