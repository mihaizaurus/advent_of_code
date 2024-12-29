use advent_of_code_2024::calendar::day11;

#[test]
fn test_day11() {
    let input_path = format!("inputs/test/day11.txt"); 
    let output_path = format!("results/test/day11.txt");

    let (part1,part2) = day11::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 55312, "part 1 mismatch for day 11");
    assert_eq!(part2, 65601038650482, "part 2 mismatch for day 11");
}