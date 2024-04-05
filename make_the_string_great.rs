pub fn make_good(s: String) -> String {
    let mut stack: Vec<u8> = Vec::new();
    let diff = b'A'.abs_diff(b'a');

    for b in s.as_bytes() {
        if stack.is_empty() {
            stack.push(*b);
        } else if stack.last().unwrap().abs_diff(*b) == diff {
            stack.pop();
        } else {
            stack.push(*b);
        }
    }

    return String::from_utf8(stack).unwrap();
}

#[test]
fn tests() {
    let s = String::from("leEeetcode");
    assert_eq!(String::from("leetcode"), make_good(s));

    let s = String::from("abBAcC");
    assert_eq!(String::from(""), make_good(s));

    let s = String::from("s");
    assert_eq!(String::from("s"), make_good(s));
}
