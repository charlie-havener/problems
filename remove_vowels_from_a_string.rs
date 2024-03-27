pub fn remove_vowels(s: String) -> String {
    s.chars().filter(|c| !"aeiou".contains(*c)).collect::<String>()
}

#[test]
fn tests() {
    let s = String::from("leetcodeisacommunityforcoders");
    assert_eq!(String::from("ltcdscmmntyfrcdrs"), remove_vowels(s));

    let s = String::from("aeiou");
    assert_eq!(String::from(""), remove_vowels(s));

    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    assert_eq!(String::from("bcdfghjklmnpqrstvwxyz"), remove_vowels(s));
}
