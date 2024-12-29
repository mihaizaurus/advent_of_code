use core::fmt;

pub type Position = (usize, usize);
pub type Direction = (isize, isize);
pub type Edge = (Position, Direction, Option<Position>);
pub type PuzzleAnswer = (isize, isize);
pub type SimpleGrid = Vec<String>;

pub const DIRECTIONS: [Direction;4] = [
    (0,-1),     // Up
    (1,0),      // Right
    (0,1),      // Down
    (-1,0),     // Left
];

pub const DIAGONALS: [Direction;4] = [
    (-1,-1),    // Up-Left Diagonal
    (1,-1),     // Up-Right Diagonal
    (1,1),      // Down-Right Diagonal
    (-1,1),     // Down-Left Diagonal
];

pub struct Machine {
    button_a: Direction,
    button_b: Direction,
    prize_location: Position
}

impl Machine {
    pub fn new(button_a: Direction, button_b: Direction, prize_location: Position) -> Machine {
        Machine {
            button_a,
            button_b,
            prize_location
        }
    }

    pub fn get_buttons(&self) -> (Direction, Direction) {
        (self.button_a, self.button_b)
    }

    pub fn get_prize_location(&self) -> Position {
        self.prize_location
    }

    pub fn update_prize_location(&mut self) -> &Self {
        let prize_location = (self.prize_location.0 + 10000000000000, self.prize_location.1 + 10000000000000);
        self.prize_location = prize_location;
        self
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Machine: Button A: {:?}, Button B: {:?}, Prize Location: {:?}", self.button_a, self.button_b, self.prize_location)
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Machine: Button A: {:?}, Button B: {:?}, Prize Location: {:?}", self.button_a, self.button_b, self.prize_location)
    }
}

#[derive(Clone)]
pub struct Robot {
    position: Position,
    velocity: Direction
}

impl Robot {
    pub fn new(position: Position, velocity: Direction) -> Robot {
        Robot {
            position,
            velocity
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_velocity(&self) -> Direction {
        self.velocity
    }

    pub fn update_position(&mut self, map_size: (usize, usize)) -> &Self {
        // TODO check wraparound and correct position shift
        let (map_x, map_y) = map_size;
        let mut new_x = self.position.0 as isize + self.velocity.0;
        let mut new_y = self.position.1 as isize + self.velocity.1;

        if new_x >= map_x as isize {
            new_x = new_x - map_x as isize;
        } else if new_x < 0 {
            new_x = new_x + map_x as isize;
        }

        if new_y >= map_y as isize {
            new_y = new_y - map_y as isize;
        } else if new_y < 0 {
            new_y = new_y + map_y as isize;
        }

        self.position = (new_x as usize, new_y as usize);
        self
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Robot starting at {:?} and moving with velocity {:?}", self.position, self.velocity)
    }
}

impl fmt::Debug for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Robot starting at {:?} and moving with velocity {:?}", self.position, self.velocity)
    }
}