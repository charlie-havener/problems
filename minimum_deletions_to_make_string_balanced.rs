pub fn minimum_deletions(s: String) -> i32 {
    
    let mut v = 0;
    let mut b_count = 0;
    for c in s.chars() {
        match c {
            'a' => v = (v + 1).min(b_count),
            'b' => b_count += 1,
            _ => (),
        }
    }
    return v;
}

#[test]
fn tests() {
    let s = String::from("aababbab");
    assert_eq!(2, minimum_deletions(s)); 

    let s = String::from("bbaaaaabb");
    assert_eq!(2, minimum_deletions(s)); 
}
