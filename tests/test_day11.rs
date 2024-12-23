use AdventOfCode2024::calendar::day11;

#[test]
fn test_day11() {
    let input_path = format!("inputs/day11_test.txt"); 
    let output_path = format!("results/day11_test.txt");

    let (part1,part2) = day11::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 55312, "part 1 mismatch for day 11");
    assert_eq!(part2, 0, "part 2 mismatch for day 11");
}