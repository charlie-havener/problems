pub fn number_of_special_substrings(s: String) -> i32 {
    let mut ans = 0;
    let mut occ = [-1; 26];
    let mut left = 0;
    for (idx, &b) in s.as_bytes().iter().enumerate() {
        let i = (b - b'a') as usize;
        if occ[i] >= 0 {
            left = left.max(occ[i] + 1);
        }
        ans += idx as i32 - left + 1;
        occ[i] = idx as i32;
    }

    return ans as i32;
}

#[test]
fn tests() {
    
    let s = String::from("abcd");
    assert_eq!(10, number_of_special_substrings(s));

    let s = String::from("ooo");
    assert_eq!(3, number_of_special_substrings(s));

    let s = String::from("abab");
    assert_eq!(7, number_of_special_substrings(s));

    let s = String::from("abcdabcd");
    assert_eq!(26, number_of_special_substrings(s));

    let s = String::from("afiau");
    assert_eq!(13, number_of_special_substrings(s));

    let s = String::from("fiifj");
    assert_eq!(9, number_of_special_substrings(s));

}
