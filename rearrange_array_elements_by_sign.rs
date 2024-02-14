pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
    let mut pos = Vec::with_capacity(nums.len()/2);
    let mut neg = Vec::with_capacity(nums.len()/2);
    for idx in 0..nums.len() {
        if nums[idx] >= 0 { pos.push(nums[idx]) }
        else { neg.push(nums[idx]) }
    }

    let (mut p, mut n) = (0, 0);
    for idx in 0..nums.len() {
        match idx % 2 {
            0 => {
                nums[idx] = pos[p];
                p += 1;
            },
            1 => {
                nums[idx] = neg[n];
                n += 1;
            },
            _ => unreachable!(),
        }
    }
    return nums;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = vec![3,1,-2,-5,2,-4];
        assert_eq!(vec![3,-2,1,-5,2,-4], rearrange_array(v));
        let v = vec![-1,1];
        assert_eq!(vec![1,-1], rearrange_array(v));
    }
}
