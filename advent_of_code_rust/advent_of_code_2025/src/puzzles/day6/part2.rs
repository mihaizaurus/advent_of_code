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

    let indices: Vec<usize> = raw_math_problems[raw_math_problems.len()-1]
        .bytes()
        .enumerate()
        .filter(|&(_i, b)| b != b' ')
        .map(|(i, _)| i)
        .collect();

    for i in 0..indices.len() {
        let start = indices[i];
        let end = match i {
            x if x == indices.len()-1 => {raw_math_problems[0].len()},
            _ => {indices[i+1]}
        };
        let mut x = vec![];
        for v in 0..raw_math_problems.len() - 1 {
            let val = &raw_math_problems[v][start..end];
            x.push(val);
        }
        let numbers = transpose(x);

        let symbol = raw_math_problems.last().unwrap().chars().nth(indices[i]).unwrap();
        math_problems.push((symbol,numbers));

        symbols.push(symbol);
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

fn transpose(rows: Vec<&str>) -> Vec<u64> {
    let height = rows.len();
    let width = rows[0].len();
    let mut columns = Vec::with_capacity(width);

    for col in 0..width {
        let mut s = String::with_capacity(height);
        for row in 0..height {
            // safe because all rows are guaranteed same length
            s.push(rows[row].as_bytes()[col] as char);
        }
        if !s.trim().is_empty() {
            let num = s.trim().parse::<u64>().unwrap();
            columns.push(num);
        }
    }

    columns
}