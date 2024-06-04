pub fn longest_palindrome(s: String) -> i32 {
    let mut ans = 0;
    let mut store: i64 = 0;

    for c in s.as_bytes() {
        let idx = (c - b'A') as usize;
        let check = 1 << idx;
        if store & check > 0 {
            ans += 2;
        }
        store ^= check;
    }

    if store != 0 {
        ans += 1;
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("abccccdd");
    assert_eq!(7, longest_palindrome(s));

    let s = String::from("ABCcccDD");
    assert_eq!(5, longest_palindrome(s));

    let s = String::from("a");
    assert_eq!(1, longest_palindrome(s));
}
