#[derive(Debug, Copy, Clone)]
pub enum Operations {
    Add,
    Sub,
    Mult
}

impl Operations {
    fn eval(&self, a: i32, b: i32) -> i32 {
        match self {
            Operations::Add => a + b,
            Operations::Sub => a - b,
            Operations::Mult => a * b,
        }
    }
}


fn build_vecs(expression: String) -> (Vec<i32>, Vec<Operations>) {
    let mut nums: Vec<i32> = Vec::new();
    let mut ops: Vec<Operations> = Vec::new();

    let mut exp_iter = expression.chars();

    let mut num = 0;
    while let Some(c) = exp_iter.next() {
        if c.is_numeric() {
            num = num*10 + c.to_digit(10).unwrap();
        }
        else {
            nums.push(num as i32);
            num = 0;
            let op = match c {
                '+' => Operations::Add,
                '-' => Operations::Sub,
                '*' => Operations::Mult,
                _ => unreachable!(),
            };
            ops.push(op);
        }
    }
    nums.push(num as i32);
    return (nums, ops);
}

fn recurse(mut num_stack: Vec<i32>, mut ops_stack: Vec<Operations>, nums: &[i32], ops: &[Operations]) -> Vec<i32> {
    let mut ret = vec![];
    
    // at the end. evaluate the stack to get an ans
    if ops.len() == 0 {
        println!(" -- reached an end. nums: {num_stack:?}, ops: {ops_stack:?}");
        
        while let Some(op) = ops_stack.pop() {
            let b = num_stack.pop().unwrap();
            let a = num_stack.pop().unwrap();
            num_stack.push(op.eval(a,b));
        }
        ret.push(num_stack.pop().unwrap());
        return ret;
    }

    // do the op on the top of the stack
    if ops_stack.len() > 0 {
        let b = num_stack.pop().unwrap();
        let a = num_stack.pop().unwrap();
        let o = ops_stack.pop().unwrap();
        let r = o.eval(a,b);
        num_stack.push(r);
        ret.extend(recurse(num_stack.clone(), ops_stack.clone(), nums, ops));
        
        num_stack.pop();
        num_stack.push(a);
        num_stack.push(b);
        ops_stack.push(o);
    }

    // or just add on the next one
    num_stack.push(nums[0]);
    ops_stack.push(ops[0]);
    ret.extend(recurse(num_stack, ops_stack, &nums[1..], &ops[1..]));
    
    return ret;
}


pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {

    let (nums, ops) = build_vecs(expression);
    println!("nums: {nums:?}, ops: {ops:?}");

    let num_stack = vec![nums[0]];
    let ops_stack = vec![];
    return recurse(num_stack, ops_stack, &nums[1..], &ops);
}

#[test]
fn tests() {
    let exp = String::from("2-1-1");
    println!("expression => {exp:?}");
    println!("ans => {:?}\n", diff_ways_to_compute(exp));

    let exp = String::from("2*3-4*5");
    println!("expression => {exp:?}");
    println!("ans => {:?}\n", diff_ways_to_compute(exp));

    let exp = String::from("1+1+1+1+1+1+1+1+1+11");
    println!("expression => {exp:?}");
    println!("ans => {:?}\n", diff_ways_to_compute(exp));
}
