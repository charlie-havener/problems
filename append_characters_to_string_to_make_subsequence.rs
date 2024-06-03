pub fn append_characters(s: String, t: String) -> i32 {

    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut ptr = 0;
    for idx in 0..sb.len() {
        if ptr >= tb.len() { return 0; }

        if sb[idx] == tb[ptr] {
            ptr += 1;
        }
    }

    return (tb.len() - ptr) as i32;
}

#[test]
fn tests() {
    let s = String::from("coaching");
    let t = String::from("coding");
    assert_eq!(4, append_characters(s,t));

    let s = String::from("abcde");
    let t = String::from("a");
    assert_eq!(0, append_characters(s,t));

    let s = String::from("z");
    let t = String::from("abcde");
    assert_eq!(5, append_characters(s,t));
}
