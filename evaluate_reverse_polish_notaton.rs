pub fn eval_rpn(tokens: Vec<String>) -> i32 {

    let mut stack: Vec<i32> = Vec::new(); 

    for tok in tokens {
        if let Ok(x) = i32::from_str_radix(&tok, 10) {
            stack.push(x);
        } else {
            let n2 = stack.pop().unwrap();
            let n1 = stack.pop().unwrap();
            match tok.as_str() {
                "+" => stack.push(n1+n2),
                "-" => stack.push(n1-n2),
                "*" => stack.push(n1*n2),
                "/" => stack.push(n1/n2),
                _ => unreachable!(),
            }
        }
    }
    return stack.pop().unwrap();
}


#[cfg(test)]
mod test {
    use super::eval_rpn;

    #[test]
    fn test() {
        let v = vec!["2","1","+","3","*"];
        let v = v.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        assert_eq!(9, eval_rpn(v));

        let v = vec!["4","13","5","/","+"];
        let v = v.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        assert_eq!(6, eval_rpn(v));

        let v = vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"];
        let v = v.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        assert_eq!(22, eval_rpn(v));
    }
}
