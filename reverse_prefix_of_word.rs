pub fn reverse_prefix(mut word: String, ch: char) -> String {
    let mut idx = None;
    for (i, c) in word.chars().enumerate() {
        if c == ch {
            idx = Some(i);
            break;
        }
    }

    let mut ptr1 = idx.unwrap_or(0);
    let mut ptr2 = 0;
    unsafe {
        let b = word.as_bytes_mut();
        while ptr1 > ptr2 {
            let a = b[ptr1];
            b[ptr1] = b[ptr2];
            b[ptr2] = a;

            ptr1 -= 1;
            ptr2 += 1;
        }
    }

    return word;
}

#[test]
fn tests() {
    let s = String::from("abcdefd");
    let c = 'd';
    assert_eq!(String::from("dcbaefd"), reverse_prefix(s,c));

    let s = String::from("xyxzxe");
    let c = 'z';
    assert_eq!(String::from("zxyxxe"), reverse_prefix(s,c));

    let s = String::from("abcd");
    let c = 'z';
    assert_eq!(String::from("abcd"), reverse_prefix(s,c));
}
