pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut start = 0;
    let (mut low, mut high) = (None, None);
    let mut ans = 0;

    for idx in 0..nums.len() {
        if nums[idx] < min_k || nums[idx] > max_k {
            low = None;
            high = None;
            start = idx + 1;
            continue;
        }
        if nums[idx] == min_k { low = Some(idx) }
        if nums[idx] == max_k { high = Some(idx) }
        
        if let (Some(l), Some(h)) = (low, high) {
            ans += (l.min(h) - start) + 1;
        }
    }
    return ans as i64;
}

#[test]
fn tests() {
    let v = vec![1,3,5,2,7,5];
    let (min_k, max_k) = (1,5);
    assert_eq!(2, count_subarrays(v, min_k, max_k));
    
    let v = vec![1,1,1,1];
    let (min_k, max_k) = (1,1);
    assert_eq!(10, count_subarrays(v, min_k, max_k));
    
    let v = vec![1,1,5,5];
    let (min_k, max_k) = (1,5);
    assert_eq!(4, count_subarrays(v, min_k, max_k));

    let v = vec![1,5,1,5];
    let (min_k, max_k) = (1,5);
    assert_eq!(6, count_subarrays(v, min_k, max_k));

    let v = vec![7,7,7,7];
    let (min_k, max_k) = (1,5);
    assert_eq!(0, count_subarrays(v, min_k, max_k));

    let v = vec![0,0,0,0,0];
    let (min_k, max_k) = (1,5);
    assert_eq!(0, count_subarrays(v, min_k, max_k));

    let v = vec![7,1,1,5,5,0];
    let (min_k, max_k) = (1,5);
    assert_eq!(4, count_subarrays(v, min_k, max_k));
    
}
