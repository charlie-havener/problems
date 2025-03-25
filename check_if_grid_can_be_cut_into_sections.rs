pub fn check_dir(rectangles: &mut Vec<Vec<i32>>, low: usize, high: usize) -> bool {
    
    rectangles.sort_unstable_by_key(|r| (r[low], r[high]));

    let mut end = 0;
    let mut prev = 0;
    let mut cuts = 0;

    for (idx, r) in rectangles.iter().enumerate() {
        if r[low] >= end && idx > prev {
            cuts += 1;
            prev = idx;
            if cuts >= 2 { return true }
        }
        end = end.max(r[high]);
    }
    
    return false;
}


pub fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {

    if check_dir(&mut rectangles, 0, 2) {
        return true;
    }

    if check_dir(&mut rectangles, 1, 3) {
        return true;
    }

    return false;
}
