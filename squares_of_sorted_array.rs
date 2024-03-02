pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut ptr1 = 1;
    let mut ptr2 = nums.len();
    let mut v = vec![0; nums.len()];
    
    let mut idx = ptr2 - 1;
    while ptr1 <= ptr2 {
        if nums[ptr1 - 1].abs() > nums[ptr2 - 1].abs() {
            v[idx] = nums[ptr1 - 1] * nums[ptr1 - 1];
            ptr1 += 1;
        } else {
            v[idx] = nums[ptr2 - 1] * nums[ptr2 - 1];
            ptr2 -= 1;
        }
        idx = idx.checked_sub(1).unwrap_or(0);
    }
    return v;
}
    


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = vec![-4,-1,0,3,10];
        assert_eq!(vec![0,1,9,16,100], sorted_squares(v));

        let v = vec![-7,-3,2,3,11];
        assert_eq!(vec![4,9,9,49,121], sorted_squares(v));

        let v = vec![-5,-3,-2,-1];
        assert_eq!(vec![1,4,9,25], sorted_squares(v));
    }
}
