
fn backtrack(ans: &mut Vec<usize>, used: &mut [bool; 10], pattern: &[u8]) -> bool {

    if pattern.len() == 0 { return true; }

    for idx in 1..=9 {
        
        // make sure the digit is available
        if used[idx] { continue }

        // make sure it doesn't violate pattern
        if pattern[0] == b'D' && idx >= *ans.last().unwrap() as usize { continue }
        else if pattern[0] == b'I' && idx <= *ans.last().unwrap() as usize { continue }

        used[idx] = true;
        ans.push(idx);

        if backtrack(ans, used, &pattern[1..]) {
            return true;
        }

        used[idx] = false;
        ans.pop();

    }
    
    return false;
}


pub fn smallest_number(pattern: String) -> String {

    let mut ans = Vec::with_capacity(pattern.len() + 1);
    let pattern = pattern.as_bytes();
    let mut used = [false; 10];

    for s in 1..=9 {
        ans.clear();
        ans.push(s);
        used[s] = true;
        if backtrack(&mut ans, &mut used, pattern) {
            return ans.iter().map(|d| (b'0' + *d as u8) as char).collect::<String>();
        }
        used[s] = false;
    }
    
    return "-1".into();
}

#[test]
fn tests() {

    let pattern = String::from("IIIDIDDD");
    assert_eq!(String::from("123549876"), smallest_number(pattern));

    let pattern = String::from("DDD");
    assert_eq!(String::from("4321"), smallest_number(pattern));

}
