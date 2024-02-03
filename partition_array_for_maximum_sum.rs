pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let mut ans = vec![0; arr.len() + 1];

    for i in 0..arr.len() {
        let mut k_max = arr[i];
        for j in 0..(k as usize).min(i+1) {
            k_max = arr[i-j].max(k_max);
            let curr = ans[i-j] + k_max * (j+1) as i32;
            ans[i+1] = curr.max(ans[i+1]);
        }
    }
    return *ans.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::max_sum_after_partitioning;

    #[test]
    fn test() {
        let ans = max_sum_after_partitioning(vec![1,15,7,9,2,5,10], 3);
        assert_eq!(84, ans);
        
        let ans = max_sum_after_partitioning(vec![1,4,1,5,7,3,6,1,9,9,3], 4);
        assert_eq!(83, ans);

        let ans = max_sum_after_partitioning(vec![1], 1);
        assert_eq!(1, ans);
    }
}
