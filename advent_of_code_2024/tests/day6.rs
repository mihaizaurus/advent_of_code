use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day6;

mod common;
use common::input_file_exists;

#[test]
fn day6_input_test() {
    let input_path = get_test_input_path(2024,6,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day6_part1() {
    let input_path = get_test_input_path(2024,6,None);
    let output_path = get_test_results_path(2024,6,None);
    let (part1,_) = day6::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 41, "part 1 mismatch for day 6");
}

#[test]
fn day6_part2() {
    let input_path = get_test_input_path(2024,6,None);
    let output_path = get_test_results_path(2024,6,None);
    let (_,part2) = day6::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 6, "part 2 mismatch for day 6");
}