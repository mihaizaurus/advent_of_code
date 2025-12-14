use advent_of_code_2015::puzzles::day1;
use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};

mod common;
use common::input_file_exists;

const YEAR: usize = 2025;
const DAY: usize = 1;

#[test]
fn day1_input_test() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day1_part1() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (part1, _) = day1::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part1, 3, "part 1 mismatch for DAY 1");
}

#[test]
fn day1_part2() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (_, part2) = day1::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part2, 6, "part 2 mismatch for DAY 1");
}

