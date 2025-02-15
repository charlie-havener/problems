fn partitionable(target: i32, mut curr: i32, d: &[i32]) -> bool {
    if curr > target { return false }
    if curr == target && d.len() == 0 { return true }
    let mut s = 0;
    for idx in 0..d.len() {
        s = s*10 + d[idx];
        if partitionable(target, curr + s, &d[(idx+1)..]) {
            return true;
        }
    }
    return false;
}


pub fn punishment_number(n: i32) -> i32 {
    
    let mut ans = 0;

    for v in 1..=n {
        let x = v * v;
        let d = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        if partitionable(v, 0, &d) {
            ans += x;
        }
    }
    return ans;
}

#[test]
fn tests() {
    assert_eq!(1478, punishment_number(37));

    for i in 0..=1000 {
        println!("{i}: {}", punishment_number(i));
    }
}
