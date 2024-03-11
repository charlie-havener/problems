pub fn custom_sort_string(order: String, s: String) -> String {
    let mut sorting  = vec![1; 26];

    let mut idx = -26;
    for b in order.as_bytes() {
        sorting[(b - b'a') as usize] = idx;
        idx += 1;
    }

    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable_by(
        |a,b|
        sorting[(*a) as usize - b'a' as usize].cmp(&sorting[(*b) as usize - b'a' as usize])
    );
    
    return chars.into_iter().collect::<String>();
}

#[test]
fn test() {
    let order = String::from("cba");
    let s = String::from("abcd");
    assert_eq!(String::from("cbad"), custom_sort_string(order, s));

    let order = String::from("bcafg");
    let s = String::from("abcd");
    assert_eq!(String::from("bcad"), custom_sort_string(order, s));
}
