pub fn word_pattern_match(pattern: String, s: String) -> bool {

    fn recurse(remainder: &str, pattern: &str, store: &mut Vec<String>) -> bool {

        println!("call: {:?}, {:?}", remainder, pattern);

        if pattern.len() == 0 && remainder.len() == 0 {
            //println!("{:?}", store);
            return true;
        }
        if pattern.len() == 0 || remainder.len() == 0 {
            return false;
        }

        let c = (pattern.chars().nth(0).unwrap() as u8 - b'a') as usize;
        let stored_len = store[c].len();

        if stored_len > 0 {
            if store[c].len() > remainder.len() || remainder[0..store[c].len()] != store[c] {
                println!("invalid based on previous store of {:?} : {:?}", c, store[c]);
                return false;
            }
        }

        //println!("{:?}, {:?}", remainder, pattern);
        
        for end_idx in 1..remainder.len() {
            store[c] = remainder[0..end_idx].to_string();
            //println!("{:?} = {:?}", c, store[c]);
            match recurse(&remainder[end_idx..], &pattern[1..], store) {
                true => return true,
                false => store[c] = String::new(),
            }
        }

        return false;
    }


    let mut store = vec![String::new(); 26];
    return recurse(&s, &pattern, &mut store);
}

#[test]
fn tests() {

    let pattern = String::from("abab");
    let s = String::from("redblueredblue");
    assert_eq!(true, word_pattern_match(pattern, s));

    let pattern =String::from("aaaa");
    let s = String::from("asdasdasdasd");
    assert_eq!(true, word_pattern_match(pattern, s));

    let pattern = String::from("aabb");
    let s = String::from("xyzabcxzyabc");
    assert_eq!(true, word_pattern_match(pattern, s));
}
