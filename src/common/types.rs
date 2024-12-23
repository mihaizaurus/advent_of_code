pub type Position = (usize, usize);
pub type Direction = (isize, isize);
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