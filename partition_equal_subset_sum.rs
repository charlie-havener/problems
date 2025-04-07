pub fn can_partition(nums: Vec<i32>) -> bool {

    let s = nums.iter().sum::<i32>();
    if s%2 == 1 { return false }
    let target = s / 2;
    
    let mut dp = vec![false; target as usize + 1];
    dp[0] = true;
    println!("{dp:?}");
    for v in &nums {
        for i in (*v..=target).rev() {
            dp[i as usize] = dp[i as usize] || dp[i as usize - *v as usize];
        }
        println!("{dp:?}");
        if dp[target as usize] { return true }
    }

    return false;
}

#[test]
fn tests() {
    //let nums = vec![1,5,11,5];
    //assert_eq!(true, can_partition(nums));

    //let nums = vec![1,2,3,5];
    //assert_eq!(false, can_partition(nums));

    //let nums = vec![1,1,1,2,2,2,2,3,6];
    //assert_eq!(true, can_partition(nums));

    let nums = vec![1,2,5];
    assert_eq!(false, can_partition(nums));
    
}
