pub fn minimum_length(s: String) -> i32 {
    s.as_bytes()
        .iter()
        .fold([0;26], |mut acc, b| { 
            acc[(b - b'a') as usize] += 1;
            acc
        })
        .iter()
        .filter(|c| **c > 0)
        .map(|v| (v%2) * -1 + 2)
        .sum()
}

#[test]
fn tests() {
    let s = String::from("abaacbcbb");
    assert_eq!(5, minimum_length(s));

    let s = String::from("aa");
    assert_eq!(2, minimum_length(s));

    let s = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(1, minimum_length(s));
}
