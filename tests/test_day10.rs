use AdventOfCode2024::day10;

#[test]
fn test_day10() {
    let input_path = format!("inputs/day10_test.txt"); 
    let output_path = format!("results/day10_test.txt");

    let (part1,part2) = day10::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 36, "part 1 mismatch for day 10");
    assert_eq!(part2, 81, "part 2 mismatch for day 10");
}