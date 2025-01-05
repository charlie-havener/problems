pub fn string_shift(mut s: String, shift: Vec<Vec<i32>>) -> String {
    let mut total = 0;
    for sh in shift {
        total += (sh[0] * 2 - 1) * sh[1];
    }
    // rem_euclid will be positive so usize cast is safe
    let total = total.rem_euclid(s.len() as i32) as usize;
    unsafe {
        s.as_bytes_mut().rotate_right(total);
    }

    return s;
}

#[test]
fn tests() {
    let s = String::from("abc");
    let shift = vec![vec![0,1],vec![1,2]];
    assert_eq!(String::from("cab"), string_shift(s, shift));

    let s = String::from("abcdefg");
    let shift = vec![vec![1,1],vec![1,1],vec![0,2],vec![1,3]];
    assert_eq!(String::from("efgabcd"), string_shift(s, shift));

    let s = String::from("abc");
    let shift = vec![vec![0,13],vec![0,1]];
    assert_eq!(String::from("cab"), string_shift(s, shift));

    let s = String::from("abc");
    let shift = vec![vec![1,13],vec![1,1]];
    assert_eq!(String::from("bca"), string_shift(s, shift));
}
