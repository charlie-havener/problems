pub fn check_inclusion(s1: String, s2: String) -> bool {

    let (s1, count) = s1.bytes().fold(([0; 26], 0), |mut acc, b| {
        acc.0[(b - b'a') as usize] += 1;
        acc.1 += 1;
        acc
    });

    //println!("{s1:?}");

    let mut ptr = 0;
    let mut curr = [0; 26];
    let bytes = s2.as_bytes();
    for idx in 0..bytes.len() {
        let i = (bytes[idx] - b'a') as usize;
        curr[i] += 1;

        //println!("-- {curr:?}");

        while curr[i] > s1[i] {
            curr[(bytes[ptr] - b'a') as usize] -= 1;
            ptr += 1;
        }

        //println!("{idx}, {ptr}");
        if idx as i32 - ptr as i32 + 1 == count as i32 { return true }
    }

    return false;
}


#[test]
fn tests() {
    let s1 = String::from("ab");
    let s2 = String::from("eidbaooo");
    assert!(check_inclusion(s1, s2));

    let s1 = String::from("ab");
    let s2 = String::from("eidboaoo");
    assert!(!check_inclusion(s1, s2));

    let s1 = String::from("abbc");
    let s2 = String::from("aabbabbc");
    assert!(check_inclusion(s1, s2));

    let s1 = String::from("abbc");
    let s2 = String::from("bcbatoesuth");
    assert!(check_inclusion(s1, s2));

    let s1 = String::from("abbc");
    let s2 = String::from("bckbatoesuth");
    assert!(!check_inclusion(s1, s2));
}
