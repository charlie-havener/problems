pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mut ans = -1;
    let mut running_total: i64 = nums[0] as i64 + nums[1] as i64;
    for idx in 2..nums.len() {
        if (nums[idx] as i64) < running_total {
            ans = ans.max(running_total + nums[idx] as i64);
        }
        running_total += nums[idx] as i64;
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = vec![5,5,5];
        assert_eq!(15, largest_perimeter(v));
        let v = vec![1,12,1,2,5,50,3];
        assert_eq!(12, largest_perimeter(v));
        let v = vec![5,5,50];
        assert_eq!(-1, largest_perimeter(v));
    }
}
