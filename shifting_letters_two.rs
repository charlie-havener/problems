pub fn shifting_letters(mut s: String, shifts: Vec<Vec<i32>>) -> String {

    let mut changes = vec![0; s.len()];
    for s in shifts {
        let d = s[2] * 2 - 1;
        changes[s[1] as usize] += d;
        if s[0] == 0 { continue }
        changes[s[0] as usize - 1] -= d;
    }

    // SAFETY: repalcing a single a-z char with another
    unsafe {
        let s = s.as_bytes_mut();
        let mut net_change = 0;
        for i in (0..s.len()).rev() {
            net_change += changes[i];
            let mut c = (s[i] - b'a') as i32; // 0-26
            c = (c + net_change).rem_euclid(26); // shifted char (still 0-26)
            s[i] = c as u8 + b'a';
        }
    }

    return s;
}

#[test]
fn tests() {
    let s = String::from("abc");
    let shifts = vec![vec![0,1,0],vec![1,2,1],vec![0,2,1]];
    assert_eq!(String::from("ace"), shifting_letters(s, shifts));

    let s = String::from("dztz");
    let shifts = vec![vec![0,0,0],vec![1,1,1]];
    assert_eq!(String::from("catz"), shifting_letters(s, shifts));
}
