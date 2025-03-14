use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day8;

mod common;
use common::input_file_exists;

#[test]
fn day8_input_test() {
    let input_path = get_test_input_path(2024,8,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day8_part1() {
    let input_path = get_test_input_path(2024,8,None);
    let output_path = get_test_results_path(2024,8,None);
    let (part1,_) = day8::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 14, "part 1 mismatch for day 8");
}

#[test]
fn day8_part2() {
    let input_path = get_test_input_path(2024,8,None);
    let output_path = get_test_results_path(2024,8,None);
    let (_,part2) = day8::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 34, "part 2 mismatch for day 8");
}