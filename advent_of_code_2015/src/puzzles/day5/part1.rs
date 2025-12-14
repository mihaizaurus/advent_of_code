pub fn result(ranges: &Vec<(usize,usize)>, ids: &Vec<usize>) -> isize {
    let mut fresh_ids = 0;

    for &id in ids {
        for &(min, max) in ranges {
            if min <= id && id <= max {
                fresh_ids += 1;
                break;
            }
        }
    }

    fresh_ids
}