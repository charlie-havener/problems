pub fn largest_unique_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut nums = nums.into_iter().rev();
    let mut n = (nums.next(), true);
    while let Some(v) = nums.next() {
        let prev = n.0.unwrap();
        if v == prev {
            n.1 = false;
            continue;
        }
        if v!= prev && !n.1 {
            n = (Some(v), true);
            continue;
        }
        return prev;
    }
    if n.1 { return n.0.unwrap()}
    return -1;
}

#[test]
fn tests() {
    let v = vec![5,7,3,9,4,9,8,3,1];
    assert_eq!(8, largest_unique_number(v));

    let v = vec![9,9,8,8];
    assert_eq!(-1, largest_unique_number(v));

    let v = vec![9,8,8];
    assert_eq!(9, largest_unique_number(v));

    let v = vec![9,1,9,8,8];
    assert_eq!(1, largest_unique_number(v));

    let v = vec![3];
    assert_eq!(3, largest_unique_number(v));
}
