pub fn longest_subsequence(s: String, k: i32) -> i32 {

    let mut deletions = 0;
    let mut left = 0;
    let mut ans = 0;
    let mut curr = 0;
    
    let s = s.as_bytes();

    for right in 0..s.len() {
        curr <<= 1;
        if s[right] == b'1' {
            curr ^= 1;
        }
        if curr <= k {
            ans = ans.max(right as i32 - deletions + 1);
        }
        while curr > k {
            if s[left] == b'1' {
                curr ^= 1<<(right as i32 - left as i32);
                deletions += 1;
            }
            left += 1;
        }
    }

    return ans;
}
