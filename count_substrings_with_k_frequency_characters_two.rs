pub fn number_of_substrings(s: String, k: i32) -> i64 {

    let s = s.as_bytes();

    let mut counts = [0; 26];
    let mut ans = 0;

    let mut left = 0;
    let mut right = 0;
    while right < s.len() {
        let idx = (s[right] - b'a') as usize;
        counts[idx] += 1;
        
        while counts[idx] >= k {
            ans += (s.len() - right) as i64;
            counts[(s[left] - b'a') as usize] -= 1;
            left += 1;
        }

        right += 1;
    }
    
    return ans;
}

#[test]
fn tests() {
    let s = String::from("abacb");
    let k = 2;
    assert_eq!(4, number_of_substrings(s,k));

    let s = String::from("abcde");
    let k = 1;
    assert_eq!(15, number_of_substrings(s,k));
}
