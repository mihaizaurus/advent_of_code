use advent_of_code_common::types::{Position, Direction, Edge, PuzzleAnswer, SimpleGrid, DIRECTIONS, DIAGONALS, Machine, Robot};

//Direction 1
const A: isize = 121;
const B: isize = 140;
//Direction 2
const C: isize = 95;
const D: isize = 234;
//Position 1
const V: usize = 456;
const W: usize = 236;
//Position 2
const X: usize = 330;
const Y: usize = 365;

// Position Tests

fn setup_position(x:usize, y:usize) -> Position {
    Position(x,y)
}

#[test]
fn define_position() {
    let position = setup_position(X,Y);

    assert_eq!(position.0, X, "X coordinate of Position was incorrectly defined");
    assert_eq!(position.1, Y, "Y coordinate of Position was incorrectly defined");
}

#[test]
fn get_distance_between_positions() {
    let position1 = setup_position(X,Y);
    let position2 = setup_position(V,W);
    let direction_vector_1 = position1 - position2;
    let direction_vector_2 = position2 - position1;

    assert_eq!(direction_vector_1, Direction(-126,129), "distance between two positions incorrectly defined in direction position1 - position2");
    assert_eq!(direction_vector_2, Direction(126,-129), "distance between two positions incorrectly defined in direction position2 - position1");
}

#[test]
fn display_position() {
    let position = setup_position(X,Y);
    let formatted = format!("{}",position);

    assert_eq!(formatted, "(330,365)", "Position not properly formatted for display");
}

#[test]
fn debug_position() {
    let position = setup_position(X,Y);
    let formatted = format!("{:?}",position);

    assert_eq!(formatted, "(330,365)", "Position not properly formatted for debug");
}

// Direction Tests

fn setup_direction() -> Direction {
    Direction(A,B)
}

#[test]
fn define_direction() {
    let direction = setup_direction();

    assert_eq!(direction.0, A, "X coordinate of Direction was incorrectly defined");
    assert_eq!(direction.1, B, "Y coordinate of Direction was incorrectly defined");
}

#[test]
fn display_direction() {
    let direction = setup_direction();
    let formatted = format!("{}",direction);

    assert_eq!(formatted, "(121,140)", "Direction not properly formatted for display");
}

#[test]
fn debug_direction() {
    let direction = setup_direction();
    let formatted = format!("{:?}",direction);

    assert_eq!(formatted, "(121,140)", "Direction not properly formatted for debug");
}

fn add_direction_to_direction() {
    let position = setup_position(X,Y);
    let direction = setup_direction();
    
    let new_position = position + direction;

    assert_eq!(new_position, Position(451,505), "Adding Direction to Position returns an incorrect result");
}

#[test]
fn substract_direction_from_direction() {
    let position = setup_position(X,Y);
    let direction = setup_direction();
    
    let new_position = position - direction;

    assert_eq!(new_position, Position(209,225), "Substracting Direction from Position returns an incorrect result");
}

// Combined Position-Direction Tests

#[test]
fn add_direction_to_position() {
    let position = setup_position(X,Y);
    let direction = setup_direction();
    
    let new_position = position + direction;

    assert_eq!(new_position, Position(451,505), "Adding Direction to Position returns an incorrect result");
}

#[test]
fn substract_direction_from_position() {
    let position = setup_position(X,Y);
    let direction = setup_direction();
    
    let new_position = position - direction;

    assert_eq!(new_position, Position(209,225), "Substracting Direction from Position returns an incorrect result");
}