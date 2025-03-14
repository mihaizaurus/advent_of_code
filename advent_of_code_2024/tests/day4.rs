use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day4;

mod common;
use common::input_file_exists;

#[test]
fn day4_input_test() {
    let input_path = get_test_input_path(2024,4,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day4_part1() {
    let input_path = get_test_input_path(2024,4,None);
    let output_path = get_test_results_path(2024,4,None);
    let (part1,_) = day4::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 18, "part 1 mismatch for day 4");
}

#[test]
fn day4_part2() {
    let input_path = get_test_input_path(2024,4,None);
    let output_path = get_test_results_path(2024,4,None);
    let (_,part2) = day4::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 9, "part 2 mismatch for day 4");
}