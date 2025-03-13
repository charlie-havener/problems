pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {

    let mut diffs = vec![0; nums.len() + 1];
    let mut q = 0;
    let mut s = 0;
    
    for (idx, val) in nums.iter().enumerate() {
        while val + s + diffs[idx] > 0 {
            q += 1;

            if q > queries.len() { return - 1 }
            let (l, r, v) = (queries[q-1][0] as usize, queries[q-1][1] as usize, queries[q-1][2]);
            if r >= idx {
                diffs[l.max(idx)] -= v;
                diffs[r + 1] += v;
            }
        }
        s += diffs[idx];
    }
    return q as i32;
}
