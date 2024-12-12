use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn result() -> io::Result<()> {
    let input_path = "inputs/day5.txt";
    let output_path = "results/day5.txt";

    let (update_rules, updates) = get_input_data(input_path)?;
    let (validated_updates,disordered_updates) = validated_updates(updates, &update_rules)?;
    let reordered_updates = reordered_updates(disordered_updates, &update_rules)?;
    let validated_middle_sum = get_sum_of_middle_values(validated_updates)?;
    let reordered_middle_sum = get_sum_of_middle_values(reordered_updates)?;

    let mut file = File::create(output_path)?;
    writeln!(file, "Sum of validated updates middle values = {}",validated_middle_sum)?;
    writeln!(file, "Sum of reordered updates middle values = {}",reordered_middle_sum)?;

    Ok(())
}

fn get_input_data(path: &str) -> io::Result<(Vec<(String, String)>,Vec<Vec<String>>)>{
    let input_file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Failed to open file at {}. Details: {}", path, error);
            return Err(error);
        }
    };
    let reader = io::BufReader::new(input_file);

    let mut raw_input_data = Vec::new();
    let mut raw_update_rules = Vec::new();
    let mut update_rules = Vec::new();
    let mut raw_updates = Vec::new();
    let mut updates = Vec::new();

    for line in reader.lines() {
        let line = line?;
        raw_input_data.push(line);
    }

    let mut split_index = 0;
    for line_index in 0..raw_input_data.len() {
        if raw_input_data[line_index].as_str().is_empty() {
            split_index = line_index;
            break;
        }
    }

    raw_update_rules.extend_from_slice(&raw_input_data[0..split_index]);
    raw_updates.extend_from_slice(&raw_input_data[split_index+1..]);

    for update_rule in raw_update_rules {
        if let Some((a,b)) = update_rule.split_once('|') {
            update_rules.push((a.to_string(),b.to_string()));
        }
    }

    for raw_update in raw_updates {
        let update: Vec<String> = raw_update.split(",").map(|x| x.to_string()).collect();
        updates.push(update);
    }

    Ok((update_rules,updates))
}

fn validated_updates(updates: Vec<Vec<String>>, update_rules: &Vec<(String,String)>) -> io::Result<(Vec<Vec<String>>,Vec<Vec<String>>)> {
    let mut validated_updates = Vec::new();
    let mut disordered_updates = Vec::new();

    for update in updates {
        let mut is_really_correct = false;
        for i in 0..update.len()-1 {
            let mut is_correct = false;
            for j in i+1..update.len() {    
                if is_order_correct(&update[i], &update[j], &update_rules) {
                    is_correct = true;

                }
                else {
                    is_correct = false;
                    break;
                }            
            }
            if is_correct {
                is_really_correct = true;            
            }
            else {
                is_really_correct = false;
                break;
            }
        }
        if is_really_correct {
            validated_updates.push(update);
        }
        else {
            disordered_updates.push(update);
        }
    }

    Ok((validated_updates,disordered_updates))
}

fn is_order_correct(item_1: &str, item_2:&str, update_rules: &Vec<(String, String)>) -> bool {
    for (first, second) in update_rules {
        if item_1 == first && item_2 == second {
            
        } 
        else if item_1 == second && item_2 == first {
            return false
        }
    }
    return true
}

fn get_sum_of_middle_values(validated_updates: Vec<Vec<String>>) -> io::Result<usize> {
    let mut sum_of_middle_values = 0;
    for update in validated_updates {
        let mid = update.len() /2;
        sum_of_middle_values += update[mid].parse::<usize>().unwrap();
    }

    Ok(sum_of_middle_values)
}

fn reordered_updates(disordered_updates: Vec<Vec<String>>, update_rules: &Vec<(String,String)>) -> io::Result<Vec<Vec<String>>> {
    let mut reordered_updates = Vec::new();
    // TODO Reordering logic
    
    for mut update in disordered_updates {
        let mut is_reordered = false;
        while is_reordered == false {
            for i in 0..update.len()-1 {
                let mut is_correct = false;
                for j in i+1..update.len() {
                    if !is_order_correct(&update[i], &update[j], &update_rules) {                        
                        is_correct = false;
                        update.swap(i, j);       
                        // println!("reordered: {:?}",&update);                 
                    }
                    else {
                        is_correct = true;
                    }
                }
                if is_correct {
                    is_reordered = true;
                }
                else {
                    is_reordered = false;
                }
            }
        }
        if is_reordered {
            reordered_updates.push(update);
        }
    }

    Ok(reordered_updates)
}