use std::fs::File;
use std::io::{self,BufRead, Write};

pub fn result(input_path: &str, output_path: &str) -> io::Result<(isize, isize)> {
    let (mut list_left, mut list_right) = get_inputs(input_path)?;

    list_left.sort();
    list_right.sort();

    if list_left.len() == list_right.len() {
        let mut file = File::create(output_path)?;

        let dist = get_distance_sum(&list_left, &list_right);
        let sim = get_similarity_score(&list_left, &list_right);

        writeln!(file, "distance sum = {}",dist)?;
        writeln!(file, "similarity score = {}",sim)?;

        Ok((dist as isize, sim as isize))
    } 
    else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Mismatched list sizes: left = {}, right = {}",
                list_left.len(),
                list_right.len()
            ),
        ))
    }
}

fn get_inputs(path: &str) -> io::Result<(Vec<i32>,Vec<i32>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut list_left: Vec<i32> = Vec::new();
    let mut list_right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            list_left.push(left.parse::<i32>().unwrap());
            list_right.push(right.parse::<i32>().unwrap());
        }
    }

    Ok((list_left, list_right))
}

fn get_distance_sum(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut distance_sum = 0;

    for i in 0..left.len() {
        distance_sum += (left[i] - right[i]).abs();
    }

    return distance_sum
}

fn get_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut similarity_score:i32 = 0;

    for element in left {
        let count:i32 = right.iter().filter(|&&x| x == *element).count().try_into().unwrap();
        similarity_score += element * count;
    }

    return similarity_score;
}