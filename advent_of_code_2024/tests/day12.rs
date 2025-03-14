use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day12;

mod common;
use common::input_file_exists;

#[test]
fn day12_input_test() {
    let input_path = get_test_input_path(2024,12,None);
    assert!(input_file_exists(input_path.to_str().unwrap()));
}

#[test]
fn day12_part1() {
    let input_path = get_test_input_path(2024,12,None);
    let output_path = get_test_results_path(2024,12,None);
    let (part1,_) = day12::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part1, 1930, "part 1 mismatch for day 12");
}

#[test]
fn day12_part2() {
    let input_path = get_test_input_path(2024,12,None);
    let output_path = get_test_results_path(2024,12,None);
    let (_,part2) = day12::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();
    assert_eq!(part2, 1206, "part 2 mismatch for day 12");
}