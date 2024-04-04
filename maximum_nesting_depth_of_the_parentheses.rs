pub fn max_depth(s: String) -> i32 {
    let mut count = 0;
    let mut ans = 0;
    let mut c = s.chars();

    while let Some(w) = c.next() {
        match w {
            '(' => {
                count += 1;
                ans = ans.max(count);
            },
            ')' => count -= 1,
            _ => (),
        }
    }
    return ans;
}

#[test]
fn tests() {
    let s = String::from("(1+(2*3)+((8)/4))+1");
    assert_eq!(3, max_depth(s));
    let s = String::from("(1)+((2))+(((3)))");
    assert_eq!(3, max_depth(s));
}
