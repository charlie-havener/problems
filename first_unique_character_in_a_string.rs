use std::collections::HashMap;

fn first_uniq_char(s: String) -> i32 {
    let counts = s.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
        acc
    });

    for (idx, c) in s.chars().enumerate() {
        if let Some(1) = counts.get(&c) {
            return idx as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::first_uniq_char;

    #[test]
    fn test() {
        assert_eq!(0, first_uniq_char(String::from("leetcode")));
        assert_eq!(2, first_uniq_char(String::from("loveleetcode")));
        assert_eq!(-1, first_uniq_char(String::from("aabb")));
    }
}
