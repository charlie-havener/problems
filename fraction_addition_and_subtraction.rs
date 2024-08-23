pub fn fraction_addition(expression: String) -> String {

    // stack holds (numerator, denominator) pairs
    let mut stack: [(i32,i32);2] = [(0,0), (0,0)];

    let gcd = |mut n: i32, mut m: i32| -> i32 {
        if n == 0 || n == 0 { return 1 }
        m = m.abs();
        n = n.abs();
        while m != 0 {
            if m < n { std::mem::swap(&mut m, &mut n) }
            m = m.rem_euclid(n);
        }
        return n;
    };

    let parse_stack = |stack: &mut [(i32,i32);2]| {
        if stack[0] == (0,0) { stack[0] = stack[1]; return; }

        let mut n = stack[0].0 * stack[1].1 + stack[1].0 * stack[0].1;
        let mut d = stack[0].1 * stack[1].1;
        
        let divisor = gcd(n,d);
        
        n /= divisor;
        d /= divisor;

        if n == 0 { stack[0] = (0,1) }
        else { stack[0] = (n,d) }
        println!("{:?}", stack[0]);

    };

    
    let mut is_curr_numerator = true;
    let mut negative_flag = false;
    for c in expression.chars() {
        println!("{}", c);
        match c {
            '+' => {
                parse_stack(&mut stack);
                negative_flag = false;
                is_curr_numerator = true;
                stack[1].0 = 0;
                stack[1].1 = 0;
            },
            '-' => {
                parse_stack(&mut stack);
                negative_flag = true;
                is_curr_numerator = true;
                stack[1].0 = 0;
                stack[1].1 = 0;
            },
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let v = i32::from_str_radix(&c.to_string(), 10).unwrap();
                if is_curr_numerator {
                    stack[1].0 = stack[1].0 * 10 + v;
                } else {
                    stack[1].1 = stack[1].1 * 10 + v;
                }
            },
            '/' => {
                is_curr_numerator = false;
                if negative_flag {
                    stack[1].0 *= -1;
                }
            }
            _ => unreachable!(),
        }
    }
    parse_stack(&mut stack);

    if stack[0].1 < 0 {
        stack[0].0 *= -1;
        stack[0].1 *= -1;
    }
    format!("{}/{}", stack[0].0, stack[0].1)
}

#[test]
fn test() {
    let s = String::from("-1/2+1/2");
    assert_eq!(String::from("0/1"), fraction_addition(s));

    let s = String::from("-1/2+1/2+1/3");
    assert_eq!(String::from("1/3"), fraction_addition(s));

    let s = String::from("1/3-1/2");
    assert_eq!(String::from("-1/6"), fraction_addition(s));

    let s = String::from("26/43+7/13");
    assert_eq!(String::from("639/559"), fraction_addition(s));

    let s = String::from("-4/7-3/4+2/3");
    assert_eq!(String::from("-55/84"), fraction_addition(s));

}
