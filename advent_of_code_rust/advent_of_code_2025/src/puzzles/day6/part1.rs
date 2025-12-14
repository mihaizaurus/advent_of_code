pub fn result(raw_math_problems: &Vec<String>) -> isize {
    let mut total_sum = 0;

    let math_problems = parse_math_problems(&raw_math_problems);

    for math_problem in math_problems {
        total_sum += solve(math_problem);
    }
    total_sum as isize
}

fn parse_math_problems(raw_math_problems: &Vec<String>) -> Vec<(char,Vec<u64>)> {
    let mut math_problems = Vec::new();
    let mut math_inputs: Vec<Vec<u64>> = Vec::new();
    let mut symbols: Vec<char> = Vec::new();
    for i in 0..raw_math_problems.len() {
        let line = &raw_math_problems[i];
        if i < raw_math_problems.len() -1 {
            let line_vec: Vec<u64> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
            math_inputs.push(line_vec);
        } else { // last line, symbols
            symbols = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().nth(0).unwrap())
            .collect();
        }
    }

    for j in 0..symbols.len() {
        let symbol = symbols[j];
        let mut numbers = Vec::new();
        for k in 0..math_inputs.len() {
            let val = math_inputs[k][j];
            numbers.push(val);
        }
        math_problems.push((symbol,numbers));
    }

    math_problems.into_iter().collect()
}

fn solve(math_problem: (char, Vec<u64>)) -> u64 {
    let (symbol, numbers) = math_problem;
    match symbol {
        '+' => {numbers.iter().sum()},
        '*' => {numbers.iter().fold(1, |acc, &x| acc * x)},
        _ => {0} //undefined
    }
}