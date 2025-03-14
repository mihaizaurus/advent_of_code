use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::day10;

#[test]
fn sample_test() {
    let input_path = get_test_input_path(2024,10);
    let output_path = get_test_results_path(2024,10);
    println!("input: {:?} | results: {:?}", input_path, output_path);
    // let input_path = format!("advent_of_code_inputs/2024/test/day8.txt"); 
    // let output_path = format!("advent_of_code_results/2024/test/day8.txt");

    let (part1,part2) = day10::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();

    assert_eq!(part1, 36, "part 1 mismatch for day 10");
    assert_eq!(part2, 81, "part 2 mismatch for day 10");
}