pub fn result(instructions: &String) -> isize {
    let floor: isize = instructions
        .chars()
        .map(|c| if c == '(' {1} else if c == ')' {-1} else {0})
        .sum();
    
    return floor;
}