pub fn count_substrings(s: String) -> i32 {
    let mut ans = 0;

    // odd lengths
    for i in 1..=s.len() {
        let mut left = i;
        let mut right = i;
        while left > 0 && right <= s.len() {
            if &s[(left-1)..left] != &s[(right-1)..right] {
                break;
            }
            left -= 1;
            right += 1;
            ans += 1;
        }
    }
    
    // even lengths
    for i in 2..=s.len() {
        let mut left = i-1;
        let mut right = i;
        while left > 0 && right <= s.len() {
            if &s[(left-1)..left] != &s[(right-1)..right] {
                break;
            }
            left -= 1;
            right += 1;
            ans += 1;
        }
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::count_substrings;

    #[test]
    fn test() {
        let s = String::from("aaa");
        assert_eq!(6, count_substrings(s));
        
        let s = String::from("abc");
        assert_eq!(3, count_substrings(s));

        let s = String::from("abcba");
        assert_eq!(7, count_substrings(s));

        let s = String::from("abba");
        assert_eq!(6, count_substrings(s));
    }
}
