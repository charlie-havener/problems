

fn calculate_time(keyboard: String, word: String) -> i32 {

    let mut char_positions = [0; 26];
    for (idx, key) in keyboard.bytes().enumerate() {
        char_positions[(key - b'a') as usize] = idx;
    }
    
    let mut word_iter = word.bytes();
    let mut prev = word_iter.next().unwrap(); // guaranteed to have at least 1 character.
        
    let mut ans = char_positions[(prev - b'a') as usize] as i32;

    while let Some(n) = word_iter.next() {

        let p = char_positions[(prev - b'a') as usize] as i32;
        let c = char_positions[(n - b'a') as usize] as i32;

        ans += (p - c).abs();
        prev = n;
    }

    
    return ans;
}

#[cfg(test)]
mod test {
    use super::calculate_time;

    #[test]
    fn test() {
        assert_eq!(4, calculate_time(String::from("abcdefghijklmnopqrstuvwxyz"), String::from("cba")));
        assert_eq!(73, calculate_time(String::from("pqrstuvwxyzabcdefghijklmno"), String::from("leetcode")));
    }
}
