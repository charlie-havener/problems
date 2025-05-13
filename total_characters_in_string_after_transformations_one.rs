pub fn length_after_transformations(s: String, t: i32) -> i32 {

    const MOD: i32 = 1_000_000_007;

    let mut counts: [i32; 26] = [0; 26];
    for c in s.as_bytes() {
        counts[(c - b'a') as usize] += 1;
    }

    let mut ans = s.len() as i32;
    let mut ptr = 25;
    for _ in 0..t {
        if ptr == 25 {
            counts[0] = (counts[0] + counts[ptr]) % MOD;
        } else {
            counts[ptr + 1] = (counts[ptr + 1] + counts[ptr]) % MOD;
        }
        ans = (ans + counts[ptr]) % MOD;
        if ptr == 0 { ptr = 25; }
        else { ptr -= 1; }
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("abcyy");
    let t = 2;
    assert_eq!(7, length_after_transformations(s, t));

    let s = String::from("azbk");
    let t = 1;
    assert_eq!(5, length_after_transformations(s, t));

    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let t = 100000;
    assert_eq!(704103930, length_after_transformations(s, t));
}
