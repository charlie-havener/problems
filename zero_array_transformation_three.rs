use std::collections::BinaryHeap;

pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
    
    queries.sort_unstable_by_key(|k| k[0]);
    let mut ps = vec![0; nums.len() + 1];
    let mut bh = BinaryHeap::new();
    let mut q = 0;
    let mut offset = 0;

    for i in 0..nums.len() {
        offset += ps[i];
        while q < queries.len() && queries[q][0] as usize == i {
            bh.push(queries[q][1] as usize);
            q += 1;
        }
        while offset < nums[i] && !bh.is_empty() && *bh.peek().unwrap() >= i {
            offset += 1;
            ps[bh.pop().unwrap() as usize + 1] -= 1;
        }
        if offset < nums[i] {
            return - 1;
        }
    }

    return bh.len() as i32;
}
