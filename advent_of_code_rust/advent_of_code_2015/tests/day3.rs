use advent_of_code_2015::puzzles::day3;
use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};

mod common;
use common::input_file_exists;

const YEAR: usize = 2015;
const DAY: usize = 3;

#[test]
fn day3_input_test() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day3_part1() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (part1, _) = day3::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part1, 357, "part 1 mismatch for DAY {DAY}");
}

#[test]
fn day3_part2() {
    let input_path = get_test_input_path(YEAR, DAY, None);
    let output_path = get_test_results_path(YEAR, DAY, None);
    let (_, part2) = day3::result(
        &input_path.to_str().unwrap(),
        &output_path.to_str().unwrap(),
    )
    .unwrap();
    assert_eq!(part2, 3121910778619, "part 2 mismatch for DAY {DAY}");
}

