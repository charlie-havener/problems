pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    
    let digits = digits.iter().map(|s| s.chars().next().unwrap() as char).collect::<Vec<char>>();

    let mut digit_count = 0;
    let mut nt = n;
    while nt > 0 {
        digit_count += 1;
        nt /= 10;
    }

    let mut ans = 0;
    for i in 1..digit_count {
        ans += (digits.len() as i32).pow(i);
    }

    let mut i = 0;
    let n = n.to_string();
    let mut n = n.chars();
    while i < digit_count {
        let nd = n.next().unwrap();
        let mut lesser_count = 0;
        for d in &digits {
            if *d < nd { lesser_count += 1 }
        }
        ans += lesser_count * (digits.len() as i32).pow(digit_count - i - 1);
        if !digits.contains(&nd) { break }
        i+= 1;
    }
    if i >= digit_count { ans += 1 }
    return ans;
}

#[test]
fn tests() {
    let digits = vec![String::from("1"),String::from("3"),String::from("5"),String::from("7")];
    let n = 100;
    assert_eq!(20, at_most_n_given_digit_set(digits, n));

    let digits = vec![String::from("1"),String::from("4"),String::from("9")];
    let n = 1000000000;
    assert_eq!(29523, at_most_n_given_digit_set(digits, n));

    let digits = vec![String::from("7")];
    let n = 8;
    assert_eq!(1, at_most_n_given_digit_set(digits, n));

    let digits = vec![String::from("3"),String::from("4"),String::from("5"),String::from("6")];
    let n = 64;
    assert_eq!(18, at_most_n_given_digit_set(digits, n));
}
