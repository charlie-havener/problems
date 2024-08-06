pub fn minimum_pushes(word: String) -> i32 {

    let mut letter_count = word.chars().fold(vec![0; 26], |mut acc, l| {
        let p = (l as u8) - b'a';
        acc[p as usize] += 1;
        acc
    });
    letter_count.sort_unstable_by(|a,b| b.cmp(&a));
    
    let mut ans = 0;
    let mut count = 0;

    for c in letter_count {
        if c == 0 { continue }
        ans += c * ((count/8) + 1);
        count += 1;
    }

    return ans;
}

#[test]
fn test() {
    let w = String::from("abcde");
    assert_eq!(5, minimum_pushes(w));

    let w = String::from("xyzxyzxyzxyz");
    assert_eq!(12, minimum_pushes(w));

    let w = String::from("aabbccddeeffgghhiiiiii");
    assert_eq!(24, minimum_pushes(w));
}
