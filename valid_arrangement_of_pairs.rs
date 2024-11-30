use std::collections::HashMap;

pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    // key => ((in, out), children)
    let mut hm: HashMap<i32, ((usize,usize), Vec<i32>)> = pairs
        .iter()
        .fold(HashMap::new(), |mut acc, p| {
            let (a,b) = (p[0], p[1]);
            acc.entry(a).and_modify(|v| { v.1.push(b); v.0.1 += 1; }).or_insert(((0,1), vec![b]));
            acc.entry(b).and_modify(|v| v.0.0 += 1).or_insert(((1,0), Vec::new()));
            acc
        });

    let mut start = 0;
    for (k,v) in hm.iter() {
        start = *k;
        if v.0.0 < v.0.1 {
            break;
        }
    }

    let mut epath: Vec<i32> = Vec::with_capacity(2);
    let mut stack: Vec<i32> = vec![start];
    let mut ans: Vec<Vec<i32>> = vec![vec![0,0]; pairs.len()];
    let mut ptr = pairs.len() as i32 - 1;

    while !stack.is_empty() {
        let top = stack.last().unwrap();
        let child = hm.get_mut(top).unwrap();
        match child.1.len() {
            0 => {
                epath.push(stack.pop().unwrap());
                if epath.len() == 2 {
                    ans[ptr as usize] = vec![epath[1], epath[0]];
                    ptr -= 1;
                    epath[0] = epath.pop().unwrap();
                }
            },
            _ => stack.push(child.1.pop().unwrap()),
        }
    }
    return ans;
}

#[test]
fn tests() {
    let pairs = vec![vec![5,1],vec![4,5],vec![11,9],vec![9,4]];
    println!("{:?}", valid_arrangement(pairs));

    let pairs = vec![vec![1,3],vec![3,2],vec![2,1]];
    println!("{:?}", valid_arrangement(pairs));

    let pairs = vec![vec![1,2],vec![1,3],vec![2,1]];
    println!("{:?}", valid_arrangement(pairs));
}
