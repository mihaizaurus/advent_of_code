pub fn result(instructions: &String) -> isize {
    let mut floor = 0;
    let mut position = 0;

    for idx in 0..instructions.len() {
        let c = instructions.chars().nth(idx).unwrap();
        if c == '(' {
            floor +=1;
        }
        else if c == ')' {
            floor -=1;
        }
        else {
            floor += 0
        }

        if floor == -1 { // reaches basement
            position = idx + 1;
            break;
        }
    }
    
    return position as isize;
}