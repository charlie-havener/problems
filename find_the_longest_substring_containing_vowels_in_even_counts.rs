pub fn find_the_longest_substring(s: String) -> i32 {
    
    let mut pref = 0;
    let chars = [0; 26]


}

#[test]
fn tests() {
    let s = String::from("eleetminicoworoep");
    assert_eq!(13, find_the_longest_substring(s));

    let s = String::from("leetcodeisgreat");
    assert_eq!(5, find_the_longest_substring(s));

    let s = String::from("bcbcbc");
    assert_eq!(6, find_the_longest_substring(s));

    let s = String::from("aeiou");
    assert_eq!(0, find_the_longest_substring(s));

    let s = String::from("abcdfgha");
    assert_eq!(8, find_the_longest_substring(s));

    let s = String::from("aabbbbbb");
    assert_eq!(8, find_the_longest_substring(s));
}
