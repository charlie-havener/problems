use std::collections::HashMap;

pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = 0;
    let mut hm: HashMap<u8, i32> = HashMap::new();
    let mut start = 0;

    let b = s.as_bytes();
    for (idx, c) in b.iter().enumerate() {
        hm.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        if hm.len() <= k {
            ans = ans.max((idx - start) + 1);
        } else {
            while hm.len() > k {
                let s = b[start];
                match hm.get_mut(&s) {
                    Some(1) => {hm.remove(&s);},
                    Some(v) => *v -= 1,
                    _ => unreachable!(),
                }
                start += 1;
            }
        }
    }
    return ans as i32;
}

#[test]
fn tests() {
    let s = String::from("eceba");
    let k = 2;
    assert_eq!(3, length_of_longest_substring_k_distinct(s,k));

    let s = String::from("aa");
    let k = 1;
    assert_eq!(2, length_of_longest_substring_k_distinct(s,k));

    let s = String::from("eecebba");
    let k = 2;
    assert_eq!(4, length_of_longest_substring_k_distinct(s,k));

}
