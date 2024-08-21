pub fn strange_printer(s: String) -> i32 {

    let n = s.len();
    let mut cache: Vec<Vec<Option<i32>>> = vec![vec![None; n]; n];

    fn get_min(left: usize, right: usize, s: &[u8], cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if left == right { return 1 }
        if left > right { return 0 }

        if let Some(v) = cache[left][right] { return v }

        let mut best = 1 + get_min(left + 1, right, s, cache);
        for x in (left+1)..=right {
            if s[left] == s[x] {
                best = best.min(get_min(left, x - 1, s, cache) + get_min(x + 1, right, s, cache));
            }
        }
        cache[left][right] = Some(best);
        return best;
    }
    return get_min(0, n-1, s.as_bytes(), &mut cache);
}


#[test]
fn tests() {
    let s = String::from("aaabbb");
    assert_eq!(2, strange_printer(s));

    let s = String::from("aba");
    assert_eq!(2, strange_printer(s));

    let s = String::from("aaaaaaaaaa");
    assert_eq!(1, strange_printer(s));

    let s = String::from("abababa");
    assert_eq!(4, strange_printer(s));

    let s = String::from("abcabc");
    assert_eq!(5, strange_printer(s));

    let s = String::from("tbgtgb");
    assert_eq!(4, strange_printer(s));
}
