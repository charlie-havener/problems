use std::cmp::Reverse;

pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {

    let mut groups = vec![0; nums.len() - k as usize + 1];
    let mut prev = vec![(0,Reverse(0),Reverse(0),Reverse(0)); nums.len() - k as usize + 1];
    let mut s = nums.iter().take(k as usize).sum();
    prev[0] = (s,Reverse(0),Reverse(0),Reverse(0));
    groups[0] = s;
    for i in 1..prev.len() {
        s -= nums[i-1];
        s += nums[i + k as usize - 1];
        groups[i] = s;
        prev[i] = prev[i-1].max((s, Reverse(i), Reverse(0), Reverse(0)));
    }

    let mut curr = vec![(0, Reverse(0), Reverse(0), Reverse(0)); prev.len()];
    for picked in 1..3 {
        for col in (picked * k) as usize..curr.len() {
            let mut back = prev[col - k as usize].clone();
            back.0 += groups[col];
            match picked {
                1 => back.2 = Reverse(col),
                2 => back.3 = Reverse(col),
                _ => unreachable!(),
            }
            curr[col] = curr[col - 1].max(back);
        }

        std::mem::swap(&mut prev, &mut curr);
    }
    let ans = prev.last().unwrap();
    return vec![ans.1.0 as i32, ans.2.0 as i32, ans.3.0 as i32];
}

#[test]
fn tests() {
    let nums = vec![1,2,1,2,6,7,5,1];
    let k = 2;
    assert_eq!(vec![0,3,5], max_sum_of_three_subarrays(nums, k));

    let nums = vec![1,2,1,2,1,2,1,2,1];
    let k = 2;
    assert_eq!(vec![0,2,4], max_sum_of_three_subarrays(nums, k));
}
