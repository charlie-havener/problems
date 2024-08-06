use std::collections::HashMap;

pub fn longest_repeating_substring(s: String) -> i32 {
    
    const MOD: i64 = 1_000_000_007;
    let b = s.as_bytes();
    
    let mut powers = vec![1; s.len() + 1 as usize];
    for idx in 0..s.len() {
        powers[idx + 1] = powers[idx] * 26 % MOD;
    }

    let search = |len: i32| -> bool {

        let mut hm: HashMap<i64, Vec<&str>> = HashMap::new();

        // build to len
        let mut hash: i64 = 0;
        for idx in 0..len as usize {
            hash += ((b[idx] - b'a' + 1) as i64 * powers[len as usize - idx - 1]).rem_euclid(MOD);
            hash = hash.rem_euclid(MOD);
        }
        hm.entry(hash).or_insert(vec![&s[0..len as usize]]);

        // check all the next substrings of len and get their rolling hash
        for idx in (len as usize)..(s.len()) {
            hash = ((hash - (b[idx - len as usize] - b'a' + 1) as i64 * powers[len as usize - 1]) * 26 + (b[idx] - b'a' + 1) as i64).rem_euclid(MOD);
            if let Some(matches) = hm.get(&hash) {
                for m in matches {
                    if *m == &s[(idx - len as usize + 1)..=idx]{
                        return true;
                    }
                }
            }
            hm.entry(hash).and_modify(|v| v.push(&s[(idx - len as usize + 1)..=idx])).or_insert(vec![&s[(idx - len as usize + 1)..=idx]]);
        }

        return false;
    };

    let mut answer = 0;
    let mut lo = 1;
    let mut hi = s.len();

    while lo < hi {
        let mid = (hi - lo) / 2 + lo;
        match search(mid as i32) {
            true => {
                answer = mid;
                lo = mid + 1;
            },
            false => hi = mid,
        }

    }

    return answer as i32;
}

#[test]
fn test() {
    //let s = String::from("abcd");
    //assert_eq!(0, longest_repeating_substring(s));

    //let s = String::from("abbaba");
    //assert_eq!(2, longest_repeating_substring(s));

    //let s = String::from("aabcaabdaab");
    //assert_eq!(3, longest_repeating_substring(s));

    //let s = String::from("aaaa");
    //assert_eq!(3, longest_repeating_substring(s));

    let s = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    println!("len: {}", s.len());
    assert_eq!(63, longest_repeating_substring(s));
}
