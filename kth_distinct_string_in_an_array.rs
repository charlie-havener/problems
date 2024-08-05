use std::collections::HashMap;

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {

    let hm = arr.iter().enumerate().fold(HashMap::new(), |mut acc, (idx,s)| {
        acc.entry(s).and_modify(|v| *v = -1).or_insert(idx as i32);
        acc
    });

    println!("{:?}", hm);

    let mut valids = Vec::new();
    for (_,v) in hm.into_iter() {
        if v != -1 {
            valids.push(v);
        }
    }
    if valids.len() < k as usize {
        return String::from("");
    }
    valids.sort_unstable();
    return arr[valids[k as usize - 1] as usize].clone();
}

#[test]
fn test() {
    let a = vec![String::from("d"),String::from("b"),String::from("c"),String::from("b"),String::from("c"),String::from("a")];
    let k = 2;
    assert_eq!(String::from("a"), kth_distinct(a,k));

    let a = vec![String::from("aaa"),String::from("aa"),String::from("a")];
    let k = 1;
    assert_eq!(String::from("aaa"), kth_distinct(a,k));

    let a = vec![String::from("a"),String::from("b"),String::from("a")];
    let k = 3;
    assert_eq!(String::from(""), kth_distinct(a,k));
}
