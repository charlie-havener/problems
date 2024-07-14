use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StackObj {
    None,
    Element(String, usize),
    OpeniningParenthesis,
    ClosingParenthesis(usize),
}


pub fn count_of_atoms(formula: String) -> String {
    
    let mut bts = formula.bytes();

    let mut stack = vec![];
    let mut curr = StackObj::None;


    let eval_stack = |stack: &mut Vec<StackObj>| -> () {
        if let Some(StackObj::ClosingParenthesis(v)) = stack.last() {
            let mut v = *v;
            if v == 0 { v += 1 }
            stack.pop();
            let mut updated = Vec::new();
            while let Some(StackObj::Element(e,ref mut c)) = stack.pop() {
                if *c == 0 { *c += 1 }
                updated.push(StackObj::Element(e, *c*v));
            }
            stack.append(&mut updated);
        }
    };


    while let Some(b) = bts.next() {
        match b {

            // a-z
            97..=122 => {
                if let StackObj::Element(ref mut e, _) = curr {
                    e.push(b as char);
                } else {
                    panic!("lowercase letters can't be part of anything other than an element");
                }
            },

            //0-9
            48..=57  => {
                match curr {
                    StackObj::Element(_, ref mut v) | StackObj::ClosingParenthesis(ref mut v) => *v = *v * 10 + (b as usize - 48),
                    _ => panic!("numbers can only be part of closing parenthesis and elements"),
                }
            },

            //A-Z
            65..=90 => {
                if curr != StackObj::None {
                    stack.push(StackObj::None);
                    std::mem::swap(stack.last_mut().unwrap(), &mut curr);
                    eval_stack(&mut stack);
                }
                curr = StackObj::Element(String::from(""), 0);
                if let StackObj::Element(ref mut e, _) = curr {
                    e.push(b as char);
                }
            },

            // (
            40 => {
                if curr != StackObj::None {
                    stack.push(StackObj::None);
                    std::mem::swap(stack.last_mut().unwrap(), &mut curr);
                    eval_stack(&mut stack);
                }
                curr = StackObj::OpeniningParenthesis;
            },

            // )
            41 => {
                if curr != StackObj::None {
                    stack.push(StackObj::None);
                    std::mem::swap(stack.last_mut().unwrap(), &mut curr);
                    eval_stack(&mut stack);
                }
                curr = StackObj::ClosingParenthesis(0);
            },

            _ => unreachable!(),

        }
    }
    stack.push(curr);
    eval_stack(&mut stack);


    let mut hm = HashMap::new();
    for e in stack {
        if let StackObj::Element(s,mut c) = e {
            if c == 0 { c += 1 }
            hm.entry(s).and_modify(|v| *v += c).or_insert(c);
        } else {
            panic!("how ?");
        }
    }

    let mut st = hm.into_iter().collect::<Vec<(String, usize)>>();
    st.sort();
    let ans = st.iter().fold(String::new(), |mut acc, (s, v)|  {
        acc += s;
        if *v != 1 {
            acc += &format!("{v}").to_string();
        }
        acc
    });


    return ans;
}


#[test]
fn tests() {
    
    println!("a:{}, z:{}, A:{}, Z:{}", b'a', b'z', b'A', b'Z');
    println!("(:{}, ):{}, 0:{}, 9:{}", b'(', b')', b'0', b'9');


    let s = String::from("H2O");
    assert_eq!(String::from("H2O"), count_of_atoms(s));

    let s = String::from("Mg(OH)2");
    assert_eq!(String::from("H2MgO2"), count_of_atoms(s));

    let s = String::from("K4(ON(SO3)2)2");
    assert_eq!(String::from("K4N2O14S4"), count_of_atoms(s));

    let s = String::from("Mg(H2O)N");
    assert_eq!(String::from("H2MgNO"), count_of_atoms(s));
}
