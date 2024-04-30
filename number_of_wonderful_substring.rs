use std::collections::HashMap;

pub fn wonderful_substrings(word: String) -> i64 {
    let mut hm = HashMap::new();
    hm.insert(0, 1);

    let mut ans = 0;
    let mut mask = 0;
    let b = word.as_bytes();
    for i in 0..b.len() {

        mask ^= 1 << (b[i] - b'a');

        if hm.contains_key(&mask) {
            ans += hm.get(&mask).unwrap();
            hm.entry(mask).and_modify(|v| *v += 1);
        } else {
            hm.insert(mask, 1);
        }

        for o in 0..10 {
            if hm.contains_key(&(mask ^ (1 << o))) {
                ans += hm.get(&(mask ^ (1 << o))).unwrap();
            }
        }
    }
    return ans;
}

#[test]
fn tests() {
    let s = String::from("aba");
    assert_eq!(4, wonderful_substrings(s));

    let s = String::from("aabb");
    assert_eq!(9, wonderful_substrings(s));
    
    let s = String::from("he");
    assert_eq!(2, wonderful_substrings(s));
}
