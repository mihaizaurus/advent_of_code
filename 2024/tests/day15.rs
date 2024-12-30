use advent_of_code_2024::calendar::day15;

#[test]
fn test_day15_large() {
    let input_path = format!("inputs/test/day15_large.txt"); 
    let output_path = format!("results/test/day15_large.txt");

    let (part1,part2) = day15::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 10092, "part 1 mismatch for day 15");
    assert_eq!(part2, 9021, "part 2 mismatch for day 15");
}

#[test]
fn test_day15_small() {
    let input_path = format!("inputs/test/day15_small.txt"); 
    let output_path = format!("results/test/day15_small.txt");

    let (part1,part2) = day15::result(&input_path, &output_path).unwrap();

    assert_eq!(part1, 2028, "part 1 mismatch for day 15");
    assert_eq!(part2, 0, "part 2 mismatch for day 15");
}