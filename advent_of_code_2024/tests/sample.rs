use std::env;

#[test]
fn sample_test() {
    let cwd = env::current_dir().unwrap();
    println!("current wkdir: {:?}", cwd);
    // let input_path = format!("advent_of_code_inputs/2024/test/day8.txt"); 
    // let output_path = format!("advent_of_code_results/2024/test/day8.txt");

    // let (part1,part2) = day8::result(&input_path, &output_path).unwrap();

    // assert_eq!(part1, 14, "part 1 mismatch for day 8");
    // assert_eq!(part2, 34, "part 2 mismatch for day 8");
}