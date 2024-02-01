pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    println!("nums:{:?}", nums);
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(nums.len()/3);

    for sidx in (0..nums.len()).step_by(3) {
        if nums[sidx + 2] - nums[sidx] > k { return vec![] }
        ans.push(vec![nums[sidx], nums[sidx+1], nums[sidx+2]]);
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::divide_array;

    #[test]
    fn test() {
        let ans = vec![vec![1,1,3],vec![3,4,5],vec![7,8,9]];
        let input = vec![1,3,4,8,7,9,3,5,1];
        assert_eq!(ans, divide_array(input, 2));

        let ans: Vec<Vec<i32>> = vec![]; 
        let input = vec![1,3,3,2,7,3];
        assert_eq!(ans, divide_array(input, 3));
    }
}
