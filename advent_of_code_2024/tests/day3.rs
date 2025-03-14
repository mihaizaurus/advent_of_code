use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day3;

mod common;
use common::input_file_exists;

#[test]
fn day3_input_test() {
    let input_path = get_test_input_path(2024,3,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day3_part1() {
    let input_path = get_test_input_path(2024,3,None);
    let output_path = get_test_results_path(2024,3,None);
    let (part1,_) = day3::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 161, "part 1 mismatch for day 3");
}

#[test]
fn day3_part2() {
    let input_path = get_test_input_path(2024,3,None);
    let output_path = get_test_results_path(2024,3,None);
    let (_,part2) = day3::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 48, "part 2 mismatch for day 3");
}