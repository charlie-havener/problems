pub fn min_swaps(nums: Vec<i32>) -> i32 {
    
    let ones_count = nums.iter().fold(0, |acc, n| if *n == 1 { acc + 1 } else { acc });
    
    // all 0s or a single 1
    if ones_count <= 1 { return 0 }

    // all 1s or a single 0
    if ones_count >= nums.len() - 1 { return 0 }

    let (mut tip, mut tail) = (0, ones_count - 1);
    let mut swaps_needed = nums[tip..=tail].iter().fold(0, |acc, n| if *n == 0 { acc + 1 } else { acc });
    let mut ans = swaps_needed;
    while tip < nums.len() {

        // move tail forward one
        tail = (tail + 1) % nums.len();
        if nums[tail] == 0 { swaps_needed += 1 }

        // move tip forward one
        if nums[tip] == 0 { swaps_needed -= 1 }
        tip += 1;

        ans = ans.min(swaps_needed);
    }
    return ans;
}

#[test]
fn tests() {
    let v = vec![0,1,0,1,1,0,0];
    assert_eq!(1, min_swaps(v));

    let v = vec![0,1,1,1,0,0,1,1,0];
    assert_eq!(2, min_swaps(v));

    let v = vec![1,1,0,0,1];
    assert_eq!(0, min_swaps(v));
}
