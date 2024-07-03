
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_difference(nums: Vec<i32>) -> i32 {
    
    if nums.len() <= 4 { return 0; }


    let mut ans = i32::MAX;

    let mut small: BinaryHeap<i32> = BinaryHeap::with_capacity(4);
    let mut large: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(4);

    for n in nums {
        
        // small
        if small.len() < 4 {
            small.push(n);
        } else if *small.peek().unwrap() > n {
            small.pop();
            small.push(n);
        }

        // large
        if large.len() < 4 {
            large.push(Reverse(n));
        } else if large.peek().unwrap().0 < n {
            large.pop();
            large.push(Reverse(n));
        }
    }

    let small = small.into_sorted_vec();
    let large = large.into_sorted_vec();
    for idx in 0..4 {
        ans = ans.min(large[3-idx].0 - small[idx]);
    }

    return ans;
}

#[test]
fn tests() {
    let v = vec![5,3,2,4];
    assert_eq!(0, min_difference(v));

    let v = vec![1,5,0,10,14];
    assert_eq!(1, min_difference(v));

    let v = vec![3,100,20];
    assert_eq!(0, min_difference(v));
}
