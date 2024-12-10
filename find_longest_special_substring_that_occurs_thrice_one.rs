use std::collections::HashMap;

pub fn maximum_length(s: String) -> i32 {
    
    let mut hm = HashMap::new();

    let mut run_length = 0;
    let mut prev = b' ';
    for b in s.as_bytes() {
        if *b == prev {
            run_length += 1;
            continue;
        }

        let mut count = 1;
        while run_length > 0 {
            hm.entry((prev as char, run_length)).and_modify(|c| *c += count).or_insert(count);
            run_length -= 1;
            count += 1;
        }
        run_length = 1;
        prev = *b;
    }

    println!("{hm:?}, {}, {}", prev as char, run_length);


    let mut count = 1;
    while run_length > 0 {
        hm.entry((prev as char, run_length)).and_modify(|c| *c += count).or_insert(count);
        run_length -= 1;
        count += 1;
    }

    println!("{hm:?}, {}", prev as char);

    let mut ans = -1;
    for (k,v) in hm.iter() {
        if *v >= 3 {
            ans = ans.max(k.1);
        }
    }
    return ans;

}

#[test]
fn tests() {
    let s = String::from("aaaa");
    assert_eq!(2, maximum_length(s));

    let s = String::from("abcdef");
    assert_eq!(-1, maximum_length(s));

    let s = String::from("abcaba");
    assert_eq!(1, maximum_length(s));

    let s = String::from("fafff");
    assert_eq!(1, maximum_length(s));
}

