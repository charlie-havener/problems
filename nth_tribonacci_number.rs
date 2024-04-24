pub fn tribonaccie(n: i32) -> i32 {
    if n == 0 { return 0 }
    if n == 1 { return 1 }
    if n == 2 { return 1 }
    return (3..=n).into_iter().fold((0,1,1), |acc, _| {
        (acc.1, acc.2, acc.0 + acc.1 + acc.2)
    }).2
}

#[test]
fn tests() {
    assert_eq!(4, tribonaccie(4));
    assert_eq!(1389537, tribonaccie(25));
}
