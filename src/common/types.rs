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