pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let t = n*2 - 2; // cylce length
    let x = time % t; // position in cycle
    if x < n { return x + 1; }
    return t - x + 1;
}

#[test]
fn tests() {
    let n = 4;
    assert_eq!(1, pass_the_pillow(n, 0));
    assert_eq!(2, pass_the_pillow(n, 1));
    assert_eq!(3, pass_the_pillow(n, 2));
    assert_eq!(4, pass_the_pillow(n, 3));
    assert_eq!(3, pass_the_pillow(n, 4));
    assert_eq!(2, pass_the_pillow(n, 5));
    assert_eq!(1, pass_the_pillow(n, 6));
    assert_eq!(2, pass_the_pillow(n, 7));
    assert_eq!(3, pass_the_pillow(n, 8));
    assert_eq!(4, pass_the_pillow(n, 9));
    assert_eq!(3, pass_the_pillow(n, 10));
    assert_eq!(2, pass_the_pillow(n, 11));
    assert_eq!(1, pass_the_pillow(n, 12));
    assert_eq!(3, pass_the_pillow(n, 16));
    assert_eq!(2, pass_the_pillow(n, 17));

    let n = 7;
    assert_eq!(1, pass_the_pillow(n, 0));
    assert_eq!(2, pass_the_pillow(n, 1));
    assert_eq!(3, pass_the_pillow(n, 2));
    assert_eq!(4, pass_the_pillow(n, 3));
    assert_eq!(5, pass_the_pillow(n, 4));
    assert_eq!(6, pass_the_pillow(n, 5));
    assert_eq!(7, pass_the_pillow(n, 6));
    assert_eq!(6, pass_the_pillow(n, 7));
    assert_eq!(5, pass_the_pillow(n, 8));
    assert_eq!(4, pass_the_pillow(n, 9));
    assert_eq!(3, pass_the_pillow(n, 10));
    assert_eq!(2, pass_the_pillow(n, 11));
    assert_eq!(1, pass_the_pillow(n, 12));
    assert_eq!(4, pass_the_pillow(n, 27));
    assert_eq!(2, pass_the_pillow(n, 35));
}
