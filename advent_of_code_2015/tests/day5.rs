use advent_of_code_2015::puzzles::day5;
use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};

mod common;
use common::input_file_exists;

const YEAR: usize = 2015;
const DAY: usize = 5;

#[test]
fn day5_input_test() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day5_part1() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (part1, _) = day5::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part1, 3, "part 1 mismatch for DAY {DAY}");
}

#[test]
fn day5_part2() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (_, part2) = day5::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part2, 14, "part 2 mismatch for DAY {DAY}");
}

