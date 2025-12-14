use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day14;

mod common;
use common::input_file_exists;

const YEAR: usize = 2024;
const DAY: usize = 14;

#[test]
fn day14_input_test() {
    let input_path = get_test_input_path(YEAR,DAY,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day14_part1() {
    let input_path = get_test_input_path(YEAR,DAY,None);
    let output_path = get_test_results_path(YEAR,DAY,None);
    let (part1,_) = day14::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 12, "part 1 mismatch for day 14");
}

#[test]
fn day14_part2() {
    let input_path = get_test_input_path(YEAR,DAY,None);
    let output_path = get_test_results_path(YEAR,DAY,None);
    let (_,part2) = day14::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 20, "part 2 mismatch for day 14");
}