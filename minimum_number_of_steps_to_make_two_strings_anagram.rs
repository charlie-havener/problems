use std::collections::HashMap;

fn min_steps(s: String, t: String) -> i32 {
    let mut hm = HashMap::new();
    let mut ans = 0;

    for c in s.chars() {
         hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    for c in t.chars() {
        match hm.get_mut(&c) {
            None => ans += 1,
            Some(x) => if *x > 0 { *x -= 1 } else {ans += 1},
        }
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::min_steps;

    #[test]
    fn test() {
        assert_eq!(1, min_steps(String::from("aba"), String::from("bba")));
        assert_eq!(5, min_steps(String::from("leetcode"), String::from("practice")));
        assert_eq!(0, min_steps(String::from("anagram"), String::from("mangaar")));
    }
}
