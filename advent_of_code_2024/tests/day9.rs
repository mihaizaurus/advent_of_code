use advent_of_code_2024::calendar::day9;

#[test]
fn test_day9() {
    let input_path = format!("inputs/test/day9.txt"); 
    let output_path = format!("results/test/day9.txt");

    let (part1,part2) = day9::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 1928, "part 1 mismatch for day 8");
    assert_eq!(part2, 2858, "part 2 mismatch for day 8");
}