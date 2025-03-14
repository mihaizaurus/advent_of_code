use advent_of_code_2024::calendar::day8;

#[test]
fn test_day8() {
    let input_path = format!("inputs/test/day8.txt"); 
    let output_path = format!("results/test/day8.txt");

    let (part1,part2) = day8::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 14, "part 1 mismatch for day 8");
    assert_eq!(part2, 34, "part 2 mismatch for day 8");
}