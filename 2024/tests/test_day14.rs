use advent_of_code_2024::calendar::day14;

#[test]
fn test_day14() {
    let input_path = format!("inputs/test/day14.txt"); 
    let output_path = format!("results/test/day14.txt");

    let (part1,part2) = day14::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 12, "part 1 mismatch for day 14");
    assert_eq!(part2, 0, "part 2 mismatch for day 14");
}