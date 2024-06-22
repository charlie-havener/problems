pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {

    let mut seen = vec![0;50001];
    seen[0] = 1;
    let mut ans = 0;
    let mut odd_count = 0;

    for n in nums {
        if n%2 == 1 { odd_count += 1 }
        ans += seen.get((odd_count - k) as usize).unwrap_or(&0);
        seen[odd_count as usize] += 1;
    }

    return ans;
}

#[test]
fn tests() {
    let n = vec![1,1,2,1,1];
    let k = 3;
    assert_eq!(2, number_of_subarrays(n,k));

    let n = vec![2,4,6];
    let k = 1;
    assert_eq!(0, number_of_subarrays(n,k));

    let n = vec![2,2,2,1,2,2,1,2,2,2];
    let k = 2;
    assert_eq!(16, number_of_subarrays(n,k));

    let n = vec![2,2,2,1,2,2,1,2,2,2,1,2,2];
    let k = 2;
    assert_eq!(25, number_of_subarrays(n,k));

    let n = vec![2,2,2,1,2,2,1,2,2,2,1,2,2];
    let k = 1;
    assert_eq!(36, number_of_subarrays(n,k));

    let n = vec![1];
    let k = 1;
    assert_eq!(1, number_of_subarrays(n,k));

    let n = vec![2];
    let k = 1;
    assert_eq!(0, number_of_subarrays(n,k));
}
