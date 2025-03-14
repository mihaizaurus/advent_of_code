use advent_of_code_2024::puzzle::day10;

#[test]
fn test_day10() {
    let input_path = format!("inputs/test/day10.txt"); 
    let output_path = format!("results/test/day10.txt");

    let (part1,part2) = day10::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 36, "part 1 mismatch for day 10");
    assert_eq!(part2, 81, "part 2 mismatch for day 10");
}