use std::collections::HashMap;

fn close_strings2(word1: String, word2: String) -> bool {
    // impossible if the strings are not of equal length.
    // no way to add or remove characters.
    if word1.len() != word2.len() { return false }

    let mut w1hm = HashMap::new();
    let mut w2hm = HashMap::new();
    for c in word1.chars() {
        w1hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    for c in word2.chars() {
        w2hm.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }

    // if the counts of each character are the same, then swaps will suffice
    if w1hm == w2hm { return true }

    // if there are the same number of counts for each character, then
    // replacements will get the string to a point will swaps will suffice.
    let mut countmap = HashMap::new();
    for (_, v) in &w1hm {
        countmap.entry(v).and_modify(|count| *count += 1).or_insert(1);
    }
    for (k, v) in &w2hm {
        if !w1hm.contains_key(k) { return false }
        match countmap.get_mut(v) {
            None => return false,
            Some(x) => *x -= 1,
        }
        if *countmap.get(v).unwrap() < 0 { return false }
    }

    return true;
}

fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false }

    let mut w1_freq = [0; 26];
    let mut w2_freq = w1_freq.clone();
    let mut w1_bit = 0;
    let mut w2_bit = 0;

    for (c1, c2) in word1.as_bytes().iter().zip(word2.as_bytes()) {
        w1_freq[(*c1 - b'a') as usize] += 1;
        w2_freq[(*c2 - b'a') as usize] += 1;
        w1_bit |= 1<<(c1 - b'a');
        w2_bit |= 1<<(c2 - b'a');
    }

    if w1_bit != w2_bit { return false }

    w1_freq.sort_unstable();
    w2_freq.sort_unstable();

    return w1_freq == w2_freq;
}

#[cfg(test)]
mod test {
    use super::close_strings;

    #[test]
    fn test() {
        assert_eq!(true, close_strings(String::from("abc"), String::from("bca")));
        assert_eq!(false, close_strings(String::from("a"), String::from("aa")));
        assert_eq!(true, close_strings(String::from("cabbba"), String::from("abbccc")));
        assert_eq!(false, close_strings(String::from("cabbba"), String::from("aabbss")));
        assert_eq!(false, close_strings(String::from("csbbbs"), String::from("babcba")));

    }
}
