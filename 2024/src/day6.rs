use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashSet;

pub fn result() -> io::Result<(isize, isize)> {
    let input_path = "inputs/day6.txt";
    let output_path = "results/day6.txt";

    let mut map = Grid::new(&input_path)?;
    let guard_start = map.find_guard();
    let visited: HashSet<(Position,Direction)> = map.trace_guard_steps().unwrap();
    let mut loops: HashSet<Position> = HashSet::new();
    // let mut count_loops = 0;

    for (location, _) in visited {
        // check if I loop over myself
        println!("Checking Obstacle placement for location {:?}",location);
        if &guard_start == &location {
            continue
        }
        let original_char = map.get(location).unwrap();
        map.set(location,'#');
        let new_visited = map.trace_guard_steps();
        if new_visited.is_err() {
            println!("Loop with obstacle here");
            loops.insert(location);
        }
        map.set(location, original_char);
    }

    println!("Found {} loops: {:?}",loops.len(),loops);
    
    // write_result(output_path, &map, &visited)?;

    Ok((0, loops.len() as isize))
}

type Position = (isize, isize);
type Direction = (isize, isize);

const DIRECTIONS: [Direction;4] = [
    (0,-1),     // Up
    (1,0),      // Right
    (0,1),      // Down
    (-1,0),     // Left
];

#[derive(Clone)]
struct Grid {
    data: Vec<String>,
    guard_initial: Position
}

impl Grid {
    fn new(input_path: &str) -> io::Result<Grid> {
        let file = match File::open(input_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error: Failed to open file at {}. Details: {}", input_path, error);
                return Err(error);
            }
        };
        let reader = BufReader::new(file);
        
        let mut map = Vec::new();

        for line in reader.lines() {
            let line = line?;
            map.push(line);
        }

        let mut grid = Grid {
            data: map,
            guard_initial: (0,0)
        };

        grid.guard_initial = grid.find_guard();

        Ok(grid)
    }

    fn find_guard(&self) -> Position {
        for row in 0..self.data.len() {
            for col in 0..self.data[row].len() {
                if self.data[row].chars().nth(col).unwrap() == '^' {
                    return (col as isize, row as isize)
                }
            }
        }
        return (0,0)
    }

    fn get_new_direction(&self, direction: Direction) -> Direction{
        let new_direction = match direction {
            (0,-1) => (1,0),
            (1,0) => (0,1),
            (0,1) => (-1,0),      
            (-1,0) => (0,-1),
            _ => (0,0) 
        };   
        return new_direction
    }   

    fn set(&mut self, position: Position, c: char) {
        let x = position.0 as usize;
        let y = position.1 as usize;
        if let Some(row) = self.data.get_mut(y) {
            let mut chars: Vec<char> = row.chars().collect();
            if let Some(ch) = chars.get_mut(x) {
                *ch = c;
            }
            *row = chars.into_iter().collect();
        }
    }

    fn get(&self, position: Position) -> Result<char,()> {
        let x = position.0 as usize;
        let y = position.1 as usize;
        if y >= 0 && y < self.data.len() {
            if x >=0 && x < self.data[y].len() {
                return Ok(self.data[y].chars().nth(x).unwrap())
            }
            else {
                return Err(())
            }            
        }
        else {
            return Err(())
        }
    }

    fn trace_guard_steps(&self) -> Result<HashSet<(Position,Direction)>,()> {
        let mut visited= HashSet::new();
        let mut guard_position = self.guard_initial;
        let mut guard_direction: Direction = DIRECTIONS[0];
        loop {            
            // let c = match guard_direction {
            //     (-1,0) | (1,0) => '-',
            //     (0,1) | (0,-1) => '|',
            //     _ => '.'
            // };  
            // self.set(guard_position,c);

            if visited.contains(&(guard_position,guard_direction)) {
                // I am going in a loop
                return Err(())
            }

            visited.insert((guard_position,guard_direction));            

            if self.is_obstacle_ahead(guard_position, guard_direction) {
                guard_direction = self.get_new_direction(guard_direction);                
                // self.set(guard_position,'*');
            }
            else {
                if self.is_out_of_bounds_ahead(guard_position, guard_direction) {                
                    break;
                }
                guard_position = (
                    guard_position.0 + guard_direction.0,
                    guard_position.1 + guard_direction.1
                )
            }
                         
            // if visited.iter().any(|&(x,_)| x == guard_position) {
            //     self.set(guard_position,'+');
            // }
            
        }
        Ok(visited)

    }

    fn get_potential_obstacles(&self) {

    

    }

    // fn does_obstacle_cause_loop(&self, x:isize, y:isize, (dx, dy):(isize,isize)) -> bool{
    //     let mut is_loop = false;

    // }

    fn is_obstacle_ahead(&self, position: Position, direction: Direction) -> bool {
        let (x,y) = position;
        let (dx,dy) = direction;

        if y + dy >= 0 && y + dy < self.data.len() as isize {
            if x + dx >=0 && x + dx < self.data[(y + dy) as usize].len() as isize {
                return self.data[(y + dy) as usize].chars().nth((x + dx) as usize) == Some('#');
            }
        }
        false
    }

    fn is_out_of_bounds_ahead(&self, position:Position, direction:Direction) ->bool {
        let (x,y) = position;
        let (dx,dy) = direction;

        if y + dy < 0 || y + dy >= self.data.len() as isize {
            return true
        }

        if x + dx < 0 || x + dx >= self.data[(y + dy) as usize].len() as isize {
            return true
        }

        false
    }
}

fn write_result(output_path: &str, map: &Grid, visited: &HashSet<(Position, Direction)>) -> io::Result<()> {    
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    let lines = &map.data;

    for line in lines {
        writeln!(file, "{}",line)?;
    }
    
    writeln!(file,"Guard visited {} coordinates",visited.len())?;
    // writeln!(file,"{} potential new obstacle locations found",map.potentials.len())?;

    Ok(())
}