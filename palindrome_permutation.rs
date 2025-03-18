pub fn can_permute_palindrome(s: String) -> bool {

    println!("--------");

    let mut mask = 0;
    println!("{mask:b}");

    for b in s.as_bytes() {
        mask ^= 1 << (*b - b'a');
        println!("{}: {mask:b}", *b as char);
    }

    if mask == 0 || mask & (mask - 1) == 0 {
        return true;
    }

    return false;
}


#[test]
fn tests() {
    
    println!("tst: {:b}", 0 & (0-1));

    let s = String::from("aabbcc");
    assert_eq!(true, can_permute_palindrome(s));

    let s = String::from("abbc");
    assert_eq!(false, can_permute_palindrome(s));

    let s = String::from("abbcc");
    assert_eq!(true, can_permute_palindrome(s));

}
