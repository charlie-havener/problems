pub fn compressed_string(word: String) -> String {

    let mut ans = String::new();

    let mut it = word.chars();
    let mut prev = it.next().unwrap(); // word length is >= 1
    let mut count = 1;
    
    while let Some(c) = it.next() {
        if count == 9 {
            ans.push('9');
            ans.push(prev);
            count = 1;
        }
        else if c == prev {
            count += 1;
        }
        else {
            ans.push(char::from_digit(count, 10).unwrap());
            ans.push(prev);
            count = 1;
        }
        prev = c;
    }
    ans.push(char::from_digit(count, 10).unwrap());
    ans.push(prev);
    return ans;
}   

#[test]
fn tests() {
    let s = String::from("aaaaaaaaaaaaaabb");
    assert_eq!(String::from("9a5a2b"), compressed_string(s));

    let s = String::from("abcde");
    assert_eq!(String::from("1a1b1c1d1e"), compressed_string(s));
}
