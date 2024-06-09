pub fn number_of_substrings(s: String) -> i64 {
    let mut counts = [0i64; 26];
    let mut ans: i64 = 0;
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        counts[idx] += 1;
        ans += counts[idx];
    }
    return ans;
}

#[test]
fn tests() {
    let s = String::from("abcba");
    assert_eq!(7, number_of_substrings(s));

    let s = String::from("abacad");
    assert_eq!(9, number_of_substrings(s));

    let s = String::from("a");
    assert_eq!(1, number_of_substrings(s));
}
