pub fn minimum_deletions(word: String, k: i32) -> i32 {

    let mut counts = [0; 26];
    for b in word.as_bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    let mut ans = i32::MAX;
    for low in &counts {
        let mut a = 0;
        for c in &counts {
            if c < low {
                a += *c;
            }
            else if *c > low + k {
                a += *c - (low + k);
            }
        }
        ans = a.min(ans);
    }

    return ans;
}
