pub fn reverse_parentheses(s: String) -> String {


    let bt = s.as_bytes();

    let mut matches = vec![0; s.len()];
    let mut stack = vec![];
    for (i,b) in bt.iter().enumerate() {
        match b {
            b'(' => stack.push(i),
            b')' => {
                let t = stack.pop().unwrap();
                matches[i] = t;
                matches[t] = i;
            },
            _ => (),
        }
    }

    let mut ans = String::with_capacity(s.len());
    let mut idx = 0;
    let mut dir = 1;

    while idx < s.len() {
        match bt[idx] {
            b'(' | b')' => {
                idx = matches[idx];
                dir *= -1;
            },
            c => {
                ans.push(c as char);
            }
        }
        idx = (idx as i32 + dir) as usize;
    }
    
    return ans;
}

#[test]
fn tests() {
    let s = String::from("(abcd)");
    assert_eq!(String::from("dcba"), reverse_parentheses(s));

    let s = String::from("(u(love)i)");
    assert_eq!(String::from("iloveu"), reverse_parentheses(s));

    let s = String::from("(ed(et(oc))el)");
    assert_eq!(String::from("leetcode"), reverse_parentheses(s));
}
