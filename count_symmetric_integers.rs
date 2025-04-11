pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let mut ans = 0;
    
    let is_symmetric = |x: &[u8]| -> bool {
        let a = &x[..x.len()/2].iter().map(|v| *v as i32).sum::<i32>();
        let b = &x[x.len()/2..].iter().map(|v| *v as i32).sum::<i32>();
        return a == b;
    };

    for n in low..=high {
        let s = n.to_string();
        if s.len() % 2 == 1 { continue }
        if is_symmetric(s.as_bytes()) {
            ans += 1;
        }
    }
    return ans;
}
