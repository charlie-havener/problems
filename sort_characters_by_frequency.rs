use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut hm = HashMap::new();
    for c in s.chars() {
        hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut ans = String::with_capacity(s.len());
    let mut s = hm.into_iter().collect::<Vec<_>>();
    s.sort_unstable_by(|(_,b), (_,d)| d.cmp(b));
    for (letter, value) in s {
        for _ in 0..value {
            ans.push(letter);
        }
    }
    println!("{:?}", ans);
    return ans;
}

#[cfg(test)]
mod test {
    use super::frequency_sort;

    #[test]
    fn test() {
        frequency_sort(String::from("tree"));
        frequency_sort(String::from("cccaaa"));
        frequency_sort(String::from("Aabb"));
        frequency_sort(String::from("a"));
    }
}
