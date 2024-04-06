pub fn min_remove_to_make_valid(mut s: String) -> String {
    let mut ans: String = String::new();
    let mut open = 0;
    while !s.is_empty() {
        let c = s.pop().unwrap();
        if c == ')' { open += 1; }
        else if c == '(' { open -= 1; }
        if open < 0 { open = 0; }
        else { ans.push(c); }
    }

    open = 0;
    while !ans.is_empty() {
        let c = ans.pop().unwrap();
        if c == '(' { open += 1; }
        else if c == ')' { open -= 1; }
        if open < 0 { open = 0; }
        else { s.push(c); }
    }
    return s;
}

#[test]
fn tests() {
    let s = String::from("lee(t(c)o)de)");
    assert_eq!(String::from("lee(t(c)o)de"), min_remove_to_make_valid(s));
    
    let s = String::from("a)b(c)d");
    assert_eq!(String::from("ab(c)d"), min_remove_to_make_valid(s));

    let s = String::from("))((");
    assert_eq!(String::from(""), min_remove_to_make_valid(s));

    let s = String::from("))((ab");
    assert_eq!(String::from("ab"), min_remove_to_make_valid(s));

    let s = String::from("(((ab)");
    assert_eq!(String::from("(ab)"), min_remove_to_make_valid(s));

}
