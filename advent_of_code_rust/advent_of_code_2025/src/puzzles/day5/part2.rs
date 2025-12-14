pub fn result(ranges: &Vec<(usize,usize)>) -> isize {
    let fresh_ids_ranges: Vec<(usize,usize)> = compress_ranges(ranges);
    count_ids(&fresh_ids_ranges) as isize
}

fn compress_ranges(ranges: &Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut fresh_ids_ranges: Vec<(usize,usize)> = Vec::new();
    for &(min,max) in ranges {
        // note that the ranges were sorted in the prior step before being passed to part1 and part2
        if let Some((last_min, last_max)) = fresh_ids_ranges.last_mut() {
            if min <= *last_max+1 {
                *last_max = (*last_max).max(max); 
            } else {
                fresh_ids_ranges.push((min,max));
            }
        } else {
            fresh_ids_ranges.push((min,max));
        }
    }
    
    fresh_ids_ranges.into_iter().collect()
}

fn count_ids(ranges: &Vec<(usize,usize)>) -> usize {
    let mut count = 0;
    for &(min, max) in ranges {
        count += max - min + 1;
    }
    count
}