pub fn min_swaps(s: String) -> i32 {

    let mut it = s.chars();
    let mut ans = 0;
    let mut count = 0;

    loop {
        match it.next() {
            Some('[') => count += 1,
            Some(']') => count -= 1,
            None => return ans,
            _ => unreachable!(),
        }

        if count < 0 {
            while let Some(']') = it.next_back() {}
            count = 1;
            ans += 1;
        }
    }
}

#[test]
fn tests() {
    let s = String::from("][][");
    assert_eq!(1, min_swaps(s));

    let s = String::from("]]][[[");
    assert_eq!(2, min_swaps(s));

    let s = String::from("[]");
    assert_eq!(0, min_swaps(s));
}
