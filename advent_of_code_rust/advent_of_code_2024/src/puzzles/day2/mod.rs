use std::fs::File;
use std::io::{self,BufRead, Write};

pub fn result(input_path: &str, output_path: &str) -> io::Result<(isize, isize)> {
    
    let reports_list = get_inputs(input_path)?;
    let mut file = File::create(output_path)?;
    
    let safe_reports = get_safe_reports(reports_list.clone())?;
    let dampened_safe_reports = get_dampened_safe_reports(reports_list.clone())?;

    writeln!(file, "Number of safe reports = {}",safe_reports.len())?;
    writeln!(file, "Number of dampened safe reports = {}",dampened_safe_reports.len())?;

    Ok((safe_reports.len() as isize, dampened_safe_reports.len() as isize))
}

fn get_inputs(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut reports_list: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let items: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        reports_list.push(items);
    }

    Ok(reports_list)
}

fn get_safe_reports(reports_list: Vec<Vec<i32>>) -> io::Result<Vec<Vec<i32>>> {
    let mut safe_repports = Vec::new();

    for report in reports_list {
        if is_report_safe(&report) {
            safe_repports.push(report)
        };
    }

    Ok(safe_repports)
}

fn get_dampened_safe_reports(reports_list: Vec<Vec<i32>>) -> io::Result<Vec<Vec<i32>>> {
    let mut safe_repports = Vec::new();

    for report in reports_list {
        if is_report_safe(&report) {
            safe_repports.push(report)
        } else if can_be_dampened(&report) {
            safe_repports.push(report)
        };
    }

    Ok(safe_repports)
}

fn is_report_safe(report: &[i32]) -> bool {
    let differences: Vec<i32> = report
    .windows(2)
    .map(|pair| pair[1] - pair[0])
    .collect();

    if differences.iter().any(|&diff| diff < -3 || diff == 0 || diff > 3) {
        return false // Invalid vector
    }

    if differences.iter().all(|&diff| diff > 0) {
        return true
    }
    else if differences.iter().all(|&diff| diff < 0) {
        return true
    }
    else {
        return false // Invalid vector
    }
}

fn can_be_dampened(report: &[i32]) -> bool {

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();

        modified_report.remove(i);

        if is_report_safe(&modified_report) {return true}
    }
    return false
}