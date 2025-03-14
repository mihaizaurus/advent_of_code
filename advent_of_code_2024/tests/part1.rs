use advent_of_code_common::utils::{get_test_input_path, get_test_results_path};
use advent_of_code_2024::puzzles::{day10::part1, *};

fn run_daily_test(year: usize, day: usize, expected_part1: isize, expected_part2: isize) {
    let input_path = get_test_input_path(year,day);
    let output_path = get_test_results_path(year,day);

    let (part1,part2) = match day {
        10 => day10::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap(),
        _ => panic!("No test available for the provided day: {} of year: {}",day, year)
    };
}

#[test]
fn day1_2024_part1() {
    let input_path = get_test_input_path(2024,1);
    let output_path = get_test_results_path(2024,1);
    println!("input: {:?} | results: {:?}", input_path, output_path);
    // let input_path = format!("advent_of_code_inputs/2024/test/day8.txt"); 
    // let output_path = format!("advent_of_code_results/2024/test/day8.txt");

    // let (part1,part2) = day1::result(&input_path.to_str().unwrap(), &output_path.to_str().unwrap()).unwrap();

    // assert_eq!(part1, 36, "part 1 mismatch for day 10");
    // assert_eq!(part2, 81, "part 2 mismatch for day 10");
}