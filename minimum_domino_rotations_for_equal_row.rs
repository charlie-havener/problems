pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {

    // value 'v' is stored at index 'v' - 1
    // (top_count, bottom_count, total_domino_count)
    let mut counts: [(i32, i32, i32); 6] = [(0,0,0); 6];
    for i in 0..tops.len() {

        counts[tops[i] as usize - 1].0 += 1;
        counts[bottoms[i] as usize - 1].1 += 1;

        // update totals value
        if tops[i] == bottoms[i] {
            counts[tops[i] as usize - 1].2 += 1;
        } else {
            counts[tops[i] as usize - 1].2 += 1;
            counts[bottoms[i] as usize - 1].2 += 1;
        }
    }

    let mut ans: Option<i32> = None;
    for (t, b, c) in counts {
        if c == tops.len() as i32 {
            ans = match ans {
                None => Some((c-t).min(c-b)),
                Some(v) => Some((c-t).min(c-b).min(v)),
            }
        }
    }
    
    return ans.unwrap_or(-1);
}
