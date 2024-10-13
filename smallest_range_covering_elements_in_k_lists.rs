use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {

    // elements of the heap are (value, array_idx, position_idx)
    let mut bh: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    let mut max_value = i32::MIN;
    for (idx, v) in nums.iter().enumerate() {
        let first = v[0];
        max_value = max_value.max(first);
        bh.push((Reverse(first), idx, 0));
    }
    let mut ans = vec![bh.peek().unwrap().0.0, max_value];

    while let Some((Reverse(value), array_idx, position_idx)) = bh.pop() {
        
        // update the answer if necessary
        if max_value - value < ans[1] - ans[0] {
            ans[0] = value;
            ans[1] = max_value;
        }

        // check if the element was the last of it's array
        // if it was then we are done
        if position_idx == nums[array_idx].len() - 1 { return ans }

        // add the next element to the heap and update max if greater
        let next = nums[array_idx][position_idx + 1];
        if next > max_value { max_value = next }
        bh.push((Reverse(next), array_idx, position_idx + 1));
    }

    unreachable!();
}

#[test]
fn tests() {
    let nums = vec![vec![4,10,15,24,26],vec![0,9,12,20],vec![5,18,22,30]];
    println!("{:?}", smallest_range(nums));

    let nums = vec![vec![1,2,3],vec![1,2,3],vec![1,2,3]];
    println!("{:?}", smallest_range(nums));

    let nums = vec![vec![100],vec![1,2,3],vec![1,2,3]];
    println!("{:?}", smallest_range(nums));
}
