use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

    let mut sums = vec![0; nums.len() + 1];
    let mut hm = HashMap::from([(0,1)]);
    let mut count = 0;

    for c in 1..=nums.len() {
        sums[c] = nums[c-1] + sums[c-1];
        let s = sums[c];
        count += *hm.get(&(s-k)).unwrap_or(&0);
        hm.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::subarray_sum;

    #[test]
    fn test() {
        assert_eq!(2, subarray_sum(vec![1,1,1], 2));
        assert_eq!(2, subarray_sum(vec![1,2,3], 3));
    }
}
