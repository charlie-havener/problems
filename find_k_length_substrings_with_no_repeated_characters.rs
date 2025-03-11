pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {

    let s = s.as_bytes();
    let mut ans = 0;
    let mut counts = [0; 26];
    let mut over_limit = 0;
    let mut len = 0;

    for (idx, letter) in s.iter().enumerate() {

        let i = (letter - b'a') as usize;
        if counts[i] == 1 {
            over_limit += 1;
        }
        counts[i] += 1;
        len += 1;

        if len == k {
            if over_limit == 0 {
                ans += 1;
            }
            let st = idx + 1 - k as usize;
            let st = (s[st] - b'a') as usize;
            if counts[st] == 2 {
                over_limit -= 1;
            }
            counts[st] -= 1;
            len -= 1;
        }
    }

    return ans;
}
