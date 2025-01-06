pub fn count_winning_sequences(s: String) -> i32 {
    let s = s.as_bytes();

    const MOD: i32 = 1_000_000_007;

    let mut prev: Vec<Vec<i32>> = vec![vec![0; s.len() * 2 + 1]; 3];

    if s[0] == b'F' {
        prev[0][s.len()] = 1;
        prev[1][s.len() + 1] = 1;
        prev[2][s.len() - 1] = 1;
    } else if s[0] == b'W' {
        prev[0][s.len() - 1] = 1;
        prev[1][s.len()] = 1;
        prev[2][s.len() + 1] = 1;
    } else {
        prev[0][s.len() + 1] = 1;
        prev[1][s.len() - 1] = 1;
        prev[2][s.len()] = 1;
    }

    // the current move
    for idx in 1..s.len() {

        let mut curr: Vec<Vec<i32>> = vec![vec![0; s.len() * 2 + 1]; 3];

        // one loop for each element
        for (i, e) in [b'F', b'W', b'E'].iter().enumerate() {
            
            let gain;
            if (s[idx] == b'E' && *e == b'F') || (s[idx] == b'F' && *e == b'W') || (s[idx] == b'W' && *e == b'E') {
                gain = 1;
            }
            else if (s[idx] == b'F' && *e == b'E') || (s[idx] == b'W' && *e == b'F') || (s[idx] == b'E' && *e == b'W') {
                gain = -1;
            } else {
                gain = 0;
            }
            
            for pos in (idx as i32 * -1 + s.len() as i32)..(idx as i32 + 1 + s.len() as i32) {
                curr[i][(pos + gain) as usize] = (prev[(i+1)%3][pos as usize] + prev[(i+2)%3][pos as usize]).rem_euclid(MOD);
            }

        }
        std::mem::swap(&mut prev, &mut curr);
    }

    let mut ans = 0;
    for idx in (s.len() + 1)..prev[0].len() {
        println!("{idx}");
        ans = (ans + prev[0][idx]).rem_euclid(MOD);
        ans = (ans + prev[1][idx]).rem_euclid(MOD);
        ans = (ans + prev[2][idx]).rem_euclid(MOD);
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("FFF");
    assert_eq!(3, count_winning_sequences(s));

    let s = String::from("FWEFW");
    assert_eq!(18, count_winning_sequences(s));

    let s = String::from("FFWE");
    assert_eq!(9, count_winning_sequences(s));

    let s = String::from("WFWE");
    assert_eq!(10, count_winning_sequences(s));

    let s = String::from("EFWE");
    assert_eq!(9, count_winning_sequences(s));
}
