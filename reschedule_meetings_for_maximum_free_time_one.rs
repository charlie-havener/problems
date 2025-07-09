pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut total = 0;
    for meeting in 0..start_time.len() {
        total += end_time[meeting] - start_time[meeting];
        let l = if meeting <= k as usize - 1 { 0 } else { end_time[meeting - k as usize] };
        let r = if meeting == start_time.len() - 1 { event_time } else { start_time[meeting + 1] };
        ans = ans.max(r - l - total);
        if meeting >= k as usize - 1 {
            total -= end_time[meeting - (k as usize - 1)] - start_time[meeting - (k as usize - 1)];
        };
    }
    return ans;
}
