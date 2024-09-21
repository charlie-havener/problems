pub fn shortest_palindrome(s: String) -> String {

    const P: i64 = 31;
    const M: i64 = 1_000_000_009;

    let mut hash_fwd = 0;
    let mut hash_rev = 0;
    let mut p_pow = 1;

    let mut s_idx: Option<usize> = None;

    for (idx, c) in s.chars().enumerate() {
        hash_fwd = (hash_fwd * P + (c as u8 - b'a' + 1) as i64).rem_euclid(M);
        hash_rev = (hash_rev + (c as u8 - b'a' + 1) as i64 * p_pow).rem_euclid(M);
        p_pow = (p_pow * P).rem_euclid(M);

        // can probably add in a linear string comparison to really really make sure
        // that they match in the 1 in a quadrillion chance there is a collision
        if hash_fwd == hash_rev {
            s_idx = Some(idx);
        }
    }

    let ret = match s_idx {
        None => &s,
        Some(i) => &s[i+1..],
    };
    let mut ret = ret.chars().rev().collect::<String>();
    ret.extend(s.chars());
    return ret;
}




#[test]
fn test() {
    assert_eq!(String::from("gfedcbabcdefg"), shortest_palindrome("abcdefg".into()));
    assert_eq!(String::from("fedcbabcdef"), shortest_palindrome("abcdef".into()));

    assert_eq!(String::from("fedcbaabcdef"), shortest_palindrome("aabcdef".into()));
    assert_eq!(String::from("edcbaabcde"), shortest_palindrome("aabcde".into()));

    assert_eq!(String::from("fedcbabcdef"), shortest_palindrome("babcdef".into()));
    assert_eq!(String::from("edcbabcde"), shortest_palindrome("babcde".into()));

    assert_eq!(String::from("edcbaabcde"), shortest_palindrome("baabcde".into()));
    assert_eq!(String::from("dcbaabcd"), shortest_palindrome("baabcd".into()));

    assert_eq!(String::from("edcbabcde"), shortest_palindrome("cbabcde".into()));
    assert_eq!(String::from("dcbabcd"), shortest_palindrome("cbabcd".into()));

    assert_eq!(String::from("dcbaabcd"), shortest_palindrome("cbaabcd".into()));
    assert_eq!(String::from("cbaabc"), shortest_palindrome("cbaabc".into()));

    assert_eq!(String::from("dcbabcd"), shortest_palindrome("dcbabcd".into()));

    assert_eq!(String::from("aaacecaaa"), shortest_palindrome("aacecaaa".into()));
    assert_eq!(String::from("dcbabcd"), shortest_palindrome("abcd".into()));
}
