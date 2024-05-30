pub fn num_steps(s: String) -> i32 {
    let mut carry = false;
    let mut s_iter = s[1..].chars().rev();
    let mut ans = 0;

    while let Some(c) = s_iter.next() {
        if c == '1' {
            if !carry {
                carry = true;
                ans += 2
            } else {
                ans += 1;
            }
        } else {
            if carry {
                ans += 1;
            }
            ans += 1;
        }
    }

    if &s[..=0] == "1" && carry {
        ans += 1
    }
    return ans;
}

#[test]
fn tests() {
    let s = String::from("1101");
    assert_eq!(6, num_steps(s));

    let s = String::from("10");
    assert_eq!(1, num_steps(s));

    let s = String::from("1");
    assert_eq!(0, num_steps(s));

    let s = String::from("1011");
    assert_eq!(6, num_steps(s));

    let s = String::from("11111");
    assert_eq!(6, num_steps(s));

    let s = String::from("10000");
    assert_eq!(4, num_steps(s));
}
