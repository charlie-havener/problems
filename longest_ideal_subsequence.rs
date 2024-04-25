pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    let mut v = vec![0; 26];
    for &b in s.as_bytes() {
        let idx = (b - b'a') as usize;
        let low = idx - idx.min(k as usize);
        let high = idx + (25 - idx).min(k as usize);
        v[idx] = 1 + v[low..=high].iter().max().unwrap();
    }
    return *v.iter().max().unwrap();
}

#[test]
fn tests() {
    let s = String::from("acfgbd");
    let k = 2;
    assert_eq!(4, longest_ideal_string(s,k));

    let s = String::from("abcd");
    let k = 3;
    assert_eq!(4, longest_ideal_string(s,k));

    let s = String::from("aacbcoaseudr");
    let k = 0;
    assert_eq!(3, longest_ideal_string(s,k));

    let s = String::from("aabbeoaseudr");
    let k = 1;
    assert_eq!(5, longest_ideal_string(s,k));

    let s = String::from("eduktdb");
    let k = 15;
    assert_eq!(5, longest_ideal_string(s,k));
}
