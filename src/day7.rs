use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn result() -> io::Result<()> {
    // let input_path = "inputs/day7.txt";
    let input_path = "inputs/day7.txt";
    let output_path = "results/day7.txt";

    let inputs = get_input(input_path)?;
    let mut sum = 0;
    
    for (result,operands) in inputs {
        if assert_equation(result, operands) {
            sum += result;
        }
    }
    
    write_result(output_path, sum)?;

    Ok(())
}

fn get_input(input_path: &str) -> io::Result<Vec<(usize, Vec<usize>)>>{    
    let mut inputs = Vec::new();
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open file at {}. Details: {}", input_path, error);
            return Err(error);
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line?;
        let resulting_equation: Vec<&str> = line.split(':').collect();
        let result = resulting_equation[0].parse::<usize>().unwrap();
        let operands: Vec<usize> = resulting_equation[1].split_whitespace().map(|operand| operand.parse::<usize>().unwrap()).collect();
        inputs.push((result,operands)); 
    }

    Ok(inputs)
}

fn assert_equation(result: usize, operands: Vec<usize>) -> bool {
    let mut possibilities = vec![(&operands,operands[0])];
    let operators = vec!['+','*','|'];
    for i in 1..operands.len() {      
        let mut inner_possibilities = vec![];      
        for (_,local_eq) in possibilities {            
            for operator in &operators {
                let mut temp_eq: usize = local_eq;
                match operator {
                    '+' => temp_eq += operands[i],
                    '*' => temp_eq *= operands[i],
                    '|' => temp_eq = (temp_eq.to_string() + operands[i].to_string().as_str()).parse::<usize>().unwrap(),
                    _ => ()
                }
                inner_possibilities.push((&operands,temp_eq));            
            }            
        }     
        possibilities = inner_possibilities;     
    }
    for (_,possibility) in possibilities {
        if possibility == result {
            return true
        }
    }
    return false
}

fn write_result(output_path: &str, sum: usize) -> io::Result<()> {    
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    writeln!(file,"Sum is {}", sum)?;

    Ok(())
}