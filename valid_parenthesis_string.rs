pub fn check_valid_string(s: String) -> bool {
    let mut low = 0;
    let mut high = 0;

    for c in s.chars() {
        match c {
            '(' => {
                low += 1;
                high += 1;
            }
            ')' => {
                low -= 1;
                high += 1;
            }
            '*' => {
                low -= 1;
                high += 1;
            }
            _ => unreachable!(),
        }
        if high < 0 {
            return false;
        }
        if low < 0 {
            low = 0;
        }
    }
    return low == 0;
}

#[test]
fn tests() {
    let s = String::from("()");
    assert_eq!(true, check_valid_string(s));

    let s = String::from("(*)");
    assert_eq!(true, check_valid_string(s));

    let s = String::from("(*))");
    assert_eq!(true, check_valid_string(s));

    let s = String::from("********))))))");
    assert_eq!(true, check_valid_string(s));
}
