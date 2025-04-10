pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {

    let start = start.to_string();
    let finish = finish.to_string();
    let a = calculate(&finish, &s, limit);
    let b = calculate(&start, &s, limit);
    return a - b;
}

fn calculate(x: &str, s: &str, limit: i32) -> i64 {

    if x.len() < s.len() { return 0 }
    if x.len() == s.len() {
        if x < s { return 0 } else { return 1 }
    }

    let suffix = &x[x.len() - s.len()..];
    let prefix_len = x.len() - s.len();
    let mut ans: i64 = 0;
    let x = x.as_bytes();

    for i in 0..prefix_len {
        let d = (x[i] as char).to_digit(10).unwrap() as i32;
        if d > limit {
            ans += (limit as i64 + 1).pow(prefix_len as u32 - i as u32);
            return ans;
        }
        ans += d as i64 * (limit as i64 + 1).pow(prefix_len as u32 - 1 - i as u32);
    }
    if suffix >= s {
        ans += 1;
    }
    return ans;
}
