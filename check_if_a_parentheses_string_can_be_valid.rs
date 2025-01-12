
pub fn can_be_valid(s: String, locked: String) -> bool {

    if s.len() % 2 == 1 { return false }

    let s = s.as_bytes();
    let locked = locked.as_bytes();

    let mut ul_stack: Vec<usize> = Vec::new();
    let mut l_stack: Vec<usize> = Vec::new();
    for idx in 0..s.len() {

        let c = s[idx];
        let l = locked[idx];

        let curr = match(c, l) {
            (_, b'0') => l,
            _ => c,
        };

        if curr == b')' {
            if l_stack.len() > 0 { l_stack.pop(); }
            else if ul_stack.len() > 0 { ul_stack.pop(); }
            else { return false; }
        } else if curr == b'(' {
            l_stack.push(idx);
        } else {
            ul_stack.push(idx);
        }
    }

    while let Some(i) = l_stack.pop() {
        if ul_stack.len() > 0 && ul_stack.pop().unwrap() > i { continue }
        return false;
    }

    return true;
}

#[test]
fn tests() {
    let s = String::from("))()))");
    let locked = String::from("010100");
    assert_eq!(true, can_be_valid(s, locked));
    
    let s = String::from("()()");
    let locked = String::from("0000");
    assert_eq!(true, can_be_valid(s, locked));
    
    let s = String::from(")");
    let locked = String::from("0");
    assert_eq!(false, can_be_valid(s, locked));

    let s = String::from("))))");
    let locked = String::from("0011");
    assert_eq!(true, can_be_valid(s, locked));

    let s = String::from(")(((");
    let locked = String::from("0100");
    assert_eq!(true, can_be_valid(s, locked));

    let s = String::from(")))(");
    let locked = String::from("0001");
    assert_eq!(false, can_be_valid(s, locked));

    let s = String::from("((())");
    let locked = String::from("00000");
    assert_eq!(false, can_be_valid(s, locked)); 

    let s = String::from("())()))()(()(((())(()()))))((((()())(())");
    let locked = String::from("1011101100010001001011000000110010100101");
    assert_eq!(true, can_be_valid(s, locked)); 

    let s = String::from("()))(()(()()()()(((())())((()((())");
    let locked = String::from("1100000000000010000100001000001101");
    assert_eq!(true, can_be_valid(s, locked)); 
}
