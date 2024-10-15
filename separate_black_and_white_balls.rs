pub fn minimum_steps(s: String) -> i64 {

    let mut it = s.chars();
    let mut ones_count: i64 = 0;
    let mut swaps: i64 = 0;

    while let Some(c) = it.next() {
        match c {
            '1' => ones_count += 1,
            '0' => swaps += ones_count,
            _ => unreachable!(),
        }
    }

    return swaps;
}

#[test]
fn tests() {
    let s = String::from("101");
    assert_eq!(1, minimum_steps(s));

    let s = String::from("100");
    assert_eq!(2, minimum_steps(s));

    let s = String::from("0111");
    assert_eq!(0, minimum_steps(s));

    let s = String::from("0110001100");
    assert_eq!(14, minimum_steps(s));

}
