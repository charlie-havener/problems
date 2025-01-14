pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {

    if a.len() > 64 { panic!("can't be done with bitmask") }

    let mut a_mask: i64 = 0;
    let mut b_mask: i64 = 0;
    let mut ans: Vec<i32> = Vec::with_capacity(a.len());

    for idx in 0..a.len() {
        a_mask |= 1<<(a[idx] - 1);
        b_mask |= 1<<(b[idx] - 1);
        ans.push((a_mask & b_mask).count_ones() as i32);
    }

    return ans;
}

#[test]
fn tests() {

    println!("{:b}", 6<<1);

    let a = vec![1,3,2,4];
    let b = vec![3,1,2,4];
    assert_eq!(vec![0,2,3,4], find_the_prefix_common_array(a, b));

    let a = vec![2,3,1];
    let b = vec![3,1,2];
    assert_eq!(vec![0,1,3], find_the_prefix_common_array(a, b));

    let a = vec![1,2,3,4];
    let b = vec![1,2,3,4];
    assert_eq!(vec![1,2,3,4], find_the_prefix_common_array(a, b));
}
