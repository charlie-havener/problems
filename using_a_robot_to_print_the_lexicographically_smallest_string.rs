pub fn robot_with_string(s: String) -> String {

    let mut freq = [0; 26];
    for c in s.as_bytes() {
        freq[(*c - b'a') as usize] += 1;
    }

    let mut t = Vec::new();
    let mut p = String::new();
    let mut idx = 0;

    for c in s.as_bytes() {
        t.push((*c - b'a') as usize);
        freq[(c - b'a') as usize] -= 1;
        while idx < 25 && freq[idx] == 0 {
            idx += 1;
        }
        while let Some(l) = t.last() {
            if *l <= idx {
                p.push((t.pop().unwrap() as u8 + b'a') as char); 
            }
            else {
                break;
            }
        }
    }

    return p;
}
