use advent_of_code_2024::puzzle::day14;

#[test]
fn test_day14() {
    let input_path = format!("advent_of_code_inputs/2024/test/day14.txt"); 
    let output_path = format!("advent_of_code_results/2024/test/day14.txt");

    let (part1,part2) = day14::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 12, "part 1 mismatch for day 14");
    assert_eq!(part2, 20, "part 2 mismatch for day 14");
}