pub fn clear_stars(mut s: String) -> String {

    let mut occurances = vec![vec![]; 26];
    
    // Safety: turning [a-z] into a '*' is safe
    unsafe {
        let s = s.as_bytes_mut();
        for idx in 0..s.len() {
            if s[idx] != b'*' {
                occurances[(s[idx] - b'a') as usize].push(idx);
            } else {
                for i in 0..26 {
                    if let Some(p) = occurances[i].pop() {
                        s[p] = b'*';
                        break;
                    }
                }
            }
        }
    }

    return s.chars().filter(|c| *c != '*').collect::<String>();
}
