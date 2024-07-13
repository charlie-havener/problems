pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut stack = vec![];
    let mut ans = 0;
    
    let eval_stack = |stack: &mut Vec<char>| -> i32 {
        if stack.len() < 2 { stack.clear(); return 0; }
        let mut a = 0;
        let mut top = stack.pop().unwrap();
        while let Some(next) = stack.pop() {
            if (next, top) == ('a', 'b') { a += x; }
            if (next, top) == ('b', 'a') { a += y; }
            top = next;
        }
        return a;
    };

    let priority = if x < y {'a'} else {'b'}; 
    let other = if priority == 'a' {'b'} else {'a'};


    for c in s.chars() {
        match c {
            'a' | 'b' => {
                if c == priority && stack.last() == Some(&other) {
                    stack.pop();
                    ans += x.max(y);
                }
                else {
                    stack.push(c);
                }
            },
            _ => {
                ans += eval_stack(&mut stack);
            },
        };
    }
    ans += eval_stack(&mut stack);
    return ans;
}


#[test]
fn test() {
    let s = String::from("cdbcbbaaabab");
    let x = 4;
    let y = 5;
    assert_eq!(19, maximum_gain(s, x, y));

    let s = String::from("aabbaaxybbaabb");
    let x = 5;
    let y = 4;
    assert_eq!(20, maximum_gain(s, x, y));

    let s = String::from("aabbabkbbbfvybssbtaobaaaabataaadabbbmakgabbaoapbbbbobaabvqhbbzbbkapabaavbbeghacabamdpaaqbqabbjbababmbakbaabajabasaabbwabrbbaabbafubayaazbbbaababbaaha");
    let x = 1926;
    let y = 4320;
    assert_eq!(112374, maximum_gain(s, x, y));
}
