pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {

    let n = grid.len() * grid[0].len();
    let r = grid[0][0] % x;
    let mut nums = Vec::with_capacity(n);
    
    for row in grid {
        for v in row {
            if v % x != r { return -1 }
            nums.push(v);
        }
    }

    nums.sort_unstable();
    let m = nums[nums.len() / 2];
    let mut ans = 0;
    for n in nums {
        ans += ((m - n)/x).abs();
    }

    return ans;
}
