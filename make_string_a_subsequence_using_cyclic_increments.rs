pub fn can_make_subsequence(str1: String, str2: String) -> bool {

    let mut s1 = str1.chars();
    let mut s2 = str2.chars();
    let mut search = s2.next().unwrap(); //s2.len >= 1

    while let Some(c) = s1.next() {
        
        // can move to next in str2 if c or c+m matches
        let mut nxt = std::char::from_u32(c as u32 + 1).unwrap();
        if b'z' < nxt as u8 { nxt = 'a' }

        if c == search || nxt == search {
            match s2.next() {
                Some(c) => search = c,
                None => return true,
            }
        }
    }
    return false;
}

#[test]
fn tests() {
    let str1 = String::from("abc");
    let str2 = String::from("ad");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("zc");
    let str2 = String::from("ad");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("ab");
    let str2 = String::from("d");
    assert_eq!(false, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("bcd");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("abcd");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("bcde");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("be");
    assert_eq!(true, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("abcdd");
    assert_eq!(false, can_make_subsequence(str1, str2));

    let str1 = String::from("abcd");
    let str2 = String::from("abcf");
    assert_eq!(false, can_make_subsequence(str1, str2));
}
