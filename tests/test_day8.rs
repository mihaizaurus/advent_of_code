use AdventOfCode2024::day8;

#[test]
fn test_day8() {
    let input_path = format!("inputs/day8_test.txt"); 
    let output_path = format!("results/day8_test.txt");

    let (part1,part2) = day8::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 14, "part 1 mismatch for day 8");
    assert_eq!(part2, 14, "part 2 mismatch for day 8");
}