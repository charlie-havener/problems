pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {

    let mut ans = 0;

    let mut width = 1;
    let n = nums.len();
    while width < n {
        let mut left = 0;
        while left < n {
            let mid = (n-1).min(left + width - 1);
            let right = (n-1).min(left + width*2 - 1);
            ans += count_pairs(&nums[left..=mid], &nums[mid+1..=right]);
            merge(&mut nums, left, mid, right);
            left += width * 2;
        }
        width *= 2;
    }
    println!("{nums:?}");
    return ans;
}

fn count_pairs(left: &[i32], right: &[i32]) -> i32 {
    let mut count = 0;
    let (mut i, mut j) = (0,0);
    while i < left.len() && j < right.len() {
        let d = right[j] as i64 * 2 + 1;
        if left[i] as i64 >= d {
            count += left.len() - i;
            j += 1;
        } else {
            i += 1;
        }
    }
    return count as i32;
}


fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize){
    let mut res: Vec<i32> = Vec::with_capacity(right - left + 1);
    let left_len = mid - left + 1;
    let right_len = right - mid;

    let (mut l, mut r) = (0, 0);
    while l < left_len && r < right_len {
        if nums[left + l] < nums[mid + r + 1] {
            res.push(nums[left + l]);
            l += 1;
        }
        else {
            res.push(nums[mid + r + 1]);
            r += 1;
        }
    }

    while l < left_len {
        res.push(nums[left + l]);
        l += 1;
    }

    while r < right_len {
        res.push(nums[mid + r + 1]);
        r += 1;
    }

    for i in 0..=(right - left) {
        nums[left + i] = res[i];
    }
}


#[test]
fn tests() {
    let nums = vec![1,3,2,3,1];
    assert_eq!(2, reverse_pairs(nums));

    let nums = vec![2,4,3,5,1];
    assert_eq!(3, reverse_pairs(nums));

    let nums = vec![1,2,5,6,3,4,1,2,1,6,8,4,6,2,1,6,7,1,9,8,1,2];
    assert_eq!(54, reverse_pairs(nums));

    let nums = vec![-5,-5];
    assert_eq!(1, reverse_pairs(nums));

    let nums = vec![2147483647,2147483647,2147483647,2147483647,2147483647,2147483647];
    assert_eq!(0, reverse_pairs(nums));

    let nums = vec![2147483647,2147483647,-2147483647,-2147483647,-2147483647,2147483647];
    assert_eq!(9, reverse_pairs(nums));
}
