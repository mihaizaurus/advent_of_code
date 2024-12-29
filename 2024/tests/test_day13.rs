use advent_of_code_2024::calendar::day13;

#[test]
fn test_day13() {
    let input_path = format!("inputs/test/day13.txt"); 
    let output_path = format!("results/test/day13.txt");

    let (part1,part2) = day13::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 480, "part 1 mismatch for day 13");
    assert_eq!(part2, 0, "part 2 mismatch for day 13");
}