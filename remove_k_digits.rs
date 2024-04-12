pub fn remove_kdigits(num: String, mut k: i32) -> String {
    let num = num.chars();
    let mut stack: Vec<char> = vec![];

    for c in num {
        while !stack.is_empty() && k > 0 && *stack.last().unwrap() > c {
            stack.pop();
            k -= 1;
        }
        stack.push(c);
    }
    
    for _ in 0..k {
        stack.pop();
    }
    let s = stack.into_iter().collect::<String>();
    let s = s.trim_start_matches('0').to_string();
    if s.is_empty() {
        return String::from("0");
    }
    return s;
}

#[test]
fn tests() {
    let s = String::from("1432219");
    let k = 3;
    assert_eq!(String::from("1219"), remove_kdigits(s,k));

    let s = String::from("10200");
    let k = 1;
    assert_eq!(String::from("200"), remove_kdigits(s,k));

    let s = String::from("10");
    let k = 2;
    assert_eq!(String::from("0"), remove_kdigits(s,k));

    let s = String::from("9990000000000");
    let k = 3;
    assert_eq!(String::from("0"), remove_kdigits(s,k));

    let s = String::from("1000");
    let k = 2;
    assert_eq!(String::from("0"), remove_kdigits(s,k));

    let s = String::from("1112356");
    let k = 1;
    assert_eq!(String::from("111235"), remove_kdigits(s,k));

    let s = String::from("1112356");
    let k = 3;
    assert_eq!(String::from("1112"), remove_kdigits(s,k));

    let s = String::from("000000999");
    let k = 3;
    assert_eq!(String::from("0"), remove_kdigits(s,k));

    let s = String::from("10001");
    let k = 4;
    assert_eq!(String::from("0"), remove_kdigits(s,k));
}
