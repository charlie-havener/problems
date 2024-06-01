pub fn score_of_string(s: String) -> i32 {
    let b = s.as_bytes();
    let mut ans = 0;
    for idx in 1..b.len() {
        ans += (b[idx] as i32 - b[idx - 1] as i32).abs();
    }
    return ans
}

#[test]
fn tests() {
    let s = String::from("hello");
    assert_eq!(13, score_of_string(s));
    let s = String::from("zaz");
    assert_eq!(50, score_of_string(s));
}
