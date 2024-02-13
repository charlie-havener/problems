pub fn is_palindrome(w: &str) -> bool {
    let w = w.as_bytes();
    let mut s = 0 as usize;
    let mut e = w.len() - 1 as usize;
    while s <= e {
        if w[s] != w[e] { return false }
        s += 1;
        e = e.checked_sub(1).unwrap_or(0);
    }
    return true;
}

pub fn first_palindrome(words: Vec<String>) -> String {
    for w in words.into_iter() {
        if is_palindrome(&w) { return w }
    }
    return String::from("");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ws = vec![String::from("abc"),String::from("car"),String::from("ada"),String::from("racecar"),String::from("cool")];
        assert_eq!("ada", first_palindrome(ws));
        let ws = vec![String::from("sonthusontuhsanoheu"),String::from("racecar")];
        assert_eq!("racecar", first_palindrome(ws));
        let ws = vec![String::from("def"),String::from("ghi")];
        assert_eq!("", first_palindrome(ws));

        let ws = vec![String::from("xngla"),String::from("e"),String::from("itrn"),String::from("y"),String::from("s"),String::from("pfp"),String::from("rdf")];
        assert_eq!("", first_palindrome(ws));
    }
}
