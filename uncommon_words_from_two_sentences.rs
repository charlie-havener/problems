use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {

    let mut hm:HashMap<&str, (usize, usize)> = HashMap::new();
    for w in s1.split(' ') {
        hm.entry(w).and_modify(|v| v.0 += 1).or_insert((1,0));
    }
    for w in s2.split(' ') {
        hm.entry(w).and_modify(|v| v.1 += 1).or_insert((0,1));
    }

    let mut ans: Vec<String> = Vec::new();
    for (key, val) in hm.iter() {
        if val.0 + val.1 == 1 {
            ans.push(key.to_string());
        }
    }
    return ans;
}

#[test]
fn tests() {
    let s1 = String::from("this apple is sweet");
    let s2 = String::from("this apple is sour");
    assert_eq!(vec![String::from("sweet"), String::from("sour")], uncommon_from_sentences(s1,s2));

    let s1 = String::from("apple apple");
    let s2 = String::from("banana");
    assert_eq!(vec![String::from("banana")], uncommon_from_sentences(s1,s2));
}
