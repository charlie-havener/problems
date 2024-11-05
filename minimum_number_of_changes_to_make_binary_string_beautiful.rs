pub fn min_changes(s: String) -> i32 {

    let mut ans = 0;
    
    let mut it = s.chars();
    let mut prev = it.next().unwrap(); // s has len >= 2k
    let mut count = 1;

    while let Some(c) = it.next() {
        match c == prev {
            true => {
                count += 1;
            },
            false => {
                if count % 2 == 0 {
                    count = 1
                }
                else {
                    ans += 1;
                    count = 0;
                }
            }
        }
        prev = c;
    }
    return ans;
}


#[test]
fn tests() {

    let s = String::from("1001");
    assert_eq!(2, min_changes(s));

    let s = String::from("10");
    assert_eq!(1, min_changes(s));

    let s = String::from("0000");
    assert_eq!(0, min_changes(s));

    let s = String::from("111000");
    assert_eq!(1, min_changes(s));

    let s = String::from("11100011");
    assert_eq!(1, min_changes(s));

    let s = String::from("11101111");
    assert_eq!(1, min_changes(s));

    let s = String::from("11000111");
    assert_eq!(1, min_changes(s));


}
