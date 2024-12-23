use AdventOfCode2024::calendar::day12;

#[test]
fn test_day12() {
    let input_path = format!("inputs/test/day12.txt"); 
    let output_path = format!("results/test/day12.txt");

    let (part1,part2) = day12::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 1930, "part 1 mismatch for day 12");
    assert_eq!(part2, 1206, "part 2 mismatch for day 12");
}