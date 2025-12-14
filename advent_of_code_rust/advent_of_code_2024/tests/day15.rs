use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day15;

mod common;
use common::input_file_exists;

const YEAR: usize = 2024;
const DAY: usize = 15;

#[test]
fn day15_small_input_test() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_small"));
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day15_small_part1() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_small"));
    let output_path = get_test_results_path(YEAR,DAY,Some("_small"));
    let (part1,_) = day15::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 2028, "part 1 mismatch for day 15 small example");
}

#[test]
fn day15_small_part2() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_small"));
    let output_path = get_test_results_path(YEAR,DAY,Some("_small"));
    let (_,part2) = day15::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 0, "part 2 mismatch for day 15 small example");
}

#[test]
fn day15_large_input_test() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_large"));
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day15_large_part1() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_large"));
    let output_path = get_test_results_path(YEAR,DAY,Some("_large"));
    let (part1,_) = day15::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 10092, "part 1 mismatch for day 15 large example");
}

#[test]
fn day15_large_part2() {
    let input_path = get_test_input_path(YEAR,DAY,Some("_large"));
    let output_path = get_test_results_path(YEAR,DAY,Some("_large"));
    let (_,part2) = day15::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 9021, "part 2 mismatch for day 15 large example");
}