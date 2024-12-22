use std::io;
use std::io::Write;
use std::fs::File;
use crate::common::types::PuzzleAnswer;
use crate::common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let disk_map = match get_input_as_grid(input_path) {
        Ok(input) => input[0].clone(),
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let mut blocks_part1 = Blocks::new(&disk_map);
    let mut blocks_part2 = Blocks::new(&disk_map);

    let part1 = blocks_part1.reorder().sum();
    let part2 = blocks_part2.smart_reorder().sum();

    
    let answer: PuzzleAnswer = (part1, part2);
    // let answer: PuzzleAnswer = (1928, 2858);
    write_answer(output_path, answer)?;

    Ok(answer)
}

#[derive(Debug)]
struct Blocks(Vec<Option<usize>>);

impl Blocks {
    fn new(disk_map: &str) -> Self {
        let mut representation = Vec::new();
    
        for i in 0..disk_map.len() {
            let c = disk_map.chars().nth(i).unwrap().to_digit(10).unwrap();
            if i % 2 == 0 {
                for _ in 0..c{
                    let char = i/2;
                    representation.push(Some(char));
                }            
            }
            else {
                for _ in 0..c{
                    representation.push(None);
                } 
            }
        }
    
        Blocks(representation)
    }

    fn reorder(& mut self) -> &mut Self {
        let mut values = self.0.clone();
        let mut last_index = values.len() - 1;
    
        for i in 0..values.len() {

            if values[i] == None {
                while values[last_index] == None {
                    last_index -= 1;
                }
                if last_index <= i {
                    break;
                }
                values.swap(i,last_index);
                last_index -= 1;

            }
        }
        self.0 = values;
        self
    }

    fn smart_reorder(&mut self) -> &mut Self {
        let chunks = self.chunkify();
        let mut remaining_chunks = chunks.clone();
        let mut reordered_chunks: Vec<(Option<usize>, usize, usize)> = Vec::new();

        for i in (0..chunks.len()).rev() { // woo! reverse loops
            if let (Some(file_id), file_length, start_index) = chunks[i] {
                let mut found = false;
                if i == 0 {
                    reordered_chunks.push((Some(file_id), file_length, start_index));
                    break;
                } 
                for j in 0..remaining_chunks.len() {
                    if let (None, ref mut empty_length, ref mut empty_start_index) = remaining_chunks[j] {
                        if *empty_start_index > start_index {
                            break;
                        }
                        if *empty_length >= file_length {
                            reordered_chunks.push((Some(file_id), file_length, *empty_start_index));
                            if *empty_length > file_length {// Shrink the empty chunk
                                *empty_length -= file_length;
                                *empty_start_index += file_length;
                            } else {
                                remaining_chunks[j] = (None,0,0);
                            }
                            reordered_chunks.push((None,file_length,start_index));
                            
                            found = true;
                            break;
                        }                        
                    }
                }
                if !found {
                    reordered_chunks.push((Some(file_id), file_length, start_index));
                }
            }
        }

        for chunk in remaining_chunks {
            if let (None, empty_length, empty_starting_index) = chunk {
                if empty_length > 0 {
                    reordered_chunks.push((None, empty_length, empty_starting_index));
                }
            }
        }

        reordered_chunks.sort_by(|a,b| a.2.cmp(&b.2));

        self.0 = self.reverse_chunkify(reordered_chunks);
        self
    }

    fn chunkify(&self) -> Vec<(Option<usize>, usize, usize)> {
        let mut blocks = Vec::new();
        let values = &self.0;
        
        let mut file_id = values[0];
        let mut start_index = 0;
        let mut file_size = 0;

        for (i, &value) in values.iter().enumerate() {
            if value == file_id {
                file_size += 1;
            }          
            else {
                blocks.push((file_id, file_size, start_index));
                file_id = value;
                start_index = i;
                file_size = 1;
            }
            
        }
        blocks.push((file_id, file_size, start_index));

        blocks
    }

    fn reverse_chunkify(&mut self, chunks: Vec<(Option<usize>, usize, usize)>) -> Vec<Option<usize>>{
        let mut new_blocks: Vec<Option<usize>> = Vec::new();
        for i in 0..chunks.len() {
            if let (Some(chunk_id),chunk_size, _) = chunks[i] {
                for _ in 0..chunk_size {
                    new_blocks.push(Some(chunk_id));
                }
            }
            else if let (None, chunk_size, _) = chunks[i] {
                for _ in 0..chunk_size {
                    new_blocks.push(None);
                }
            }
        }
        new_blocks
    }

    fn sum(&self) -> isize {
        let mut sum = 0;
        let values = self.0.clone();
        for i in 0..values.len() {
            if values[i] != None {
                let val = values[i].unwrap();
                sum += (val * i) as isize;
            }            
        }
        sum
    }
}

fn write_answer(output_path: &str, answer:PuzzleAnswer) -> io::Result<()> {
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    writeln!(file,"Puzzle 1 answer: {}", answer.0)?;
    writeln!(file,"Puzzle 2 answer: {}", answer.1)?;

    Ok(())
}