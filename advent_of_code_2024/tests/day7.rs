use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day7;

mod common;
use common::input_file_exists;

#[test]
fn day7_input_test() {
    let input_path = get_test_input_path(2024,7,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day7_part1() {
    let input_path = get_test_input_path(2024,7,None);
    let output_path = get_test_results_path(2024,7,None);
    let (part1,_) = day7::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 3749, "part 1 mismatch for day 7");
}

#[test]
fn day7_part2() {
    let input_path = get_test_input_path(2024,7,None);
    let output_path = get_test_results_path(2024,7,None);
    let (_,part2) = day7::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 11387, "part 2 mismatch for day 7");
}