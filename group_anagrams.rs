
use std::collections::HashMap;


pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let hm = strs.iter().fold(HashMap::new(), |mut acc,  val| {
        let mut t = val.chars().collect::<Vec<_>>();
        t.sort_unstable();
        acc.entry(t).and_modify(|idxs: &mut Vec<_>| idxs.push(val.to_owned())).or_insert(vec![val.to_owned()]);
        acc
    });
    return hm.into_values().collect();
}

#[cfg(test)]
mod test {
    use super::group_anagrams;

    #[test]
    fn test() {
        let v = vec![String::from("eat"),String::from("tea"),String::from("tan"),String::from("ate"),String::from("nat"),String::from("bat")];
        println!("{:?}", group_anagrams(v));

        let v = vec![String::from("")];
        println!("{:?}", group_anagrams(v));

        let v = vec![String::from("a")];
        println!("{:?}", group_anagrams(v));

        let v = vec![String::from("a"), String::from("a")];
        println!("{:?}", group_anagrams(v));
    }
}
