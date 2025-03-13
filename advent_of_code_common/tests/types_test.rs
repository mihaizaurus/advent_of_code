use advent_of_code_common::types::{Position, Direction, Edge, PuzzleAnswer, SimpleGrid, DIRECTIONS, DIAGONALS, Machine, Robot};

const A: isize = 121;
const B: isize = 140;

const X: usize = 330;
const Y: usize = 365;

// Position Tests

fn setup_position() -> Position {
    Position(X,Y)
}

#[test]
fn test_define_position() {
    let position = setup_position();

    assert_eq!(position.0, X, "X coordinate of Position was incorrectly defined");
    assert_eq!(position.1, Y, "Y coordinate of Position was incorrectly defined");
}

#[test]
fn test_display_position() {
    let position = setup_position();
    let formatted = format!("{}",position);

    assert_eq!(formatted, "(330,365)", "Position not properly formatted for display");
}

#[test]
fn test_debug_position() {
    let position = setup_position();
    let formatted = format!("{:?}",position);

    assert_eq!(formatted, "(330,365)", "Position not properly formatted for debug");
}

// Direction Tests

fn setup_direction() -> Direction {
    Direction(A,B)
}

#[test]
fn test_define_direction() {
    let direction = setup_direction();

    assert_eq!(direction.0, A, "X coordinate of Direction was incorrectly defined");
    assert_eq!(direction.1, B, "Y coordinate of Direction was incorrectly defined");
}

#[test]
fn test_display_direction() {
    let direction = setup_direction();
    let formatted = format!("{}",direction);

    assert_eq!(formatted, "(121,140)", "Direction not properly formatted for display");
}

#[test]
fn test_debug_direction() {
    let direction = setup_direction();
    let formatted = format!("{:?}",direction);

    assert_eq!(formatted, "(121,140)", "Direction not properly formatted for debug");
}

// Combined Position-Direction Tests

#[test]
fn test_add_direction_to_position() {
    let position = setup_position();
    let direction = setup_direction();
    
    let new_position = position + direction;

    assert_eq!(new_position, Position(451,505), "Adding Direction to Position returns an incorrect result");
}

#[test]
fn test_substract_direction_from_position() {
    let position = setup_position();
    let direction = setup_direction();
    
    let new_position = position - direction;

    assert_eq!(new_position, Position(209,225), "Substracting Direction from Position returns an incorrect result");
}