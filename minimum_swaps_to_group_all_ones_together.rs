pub fn min_swaps(data: Vec<i32>) -> i32 {
    
    let mut one_count = data.iter().filter(|v| **v == 1).count();
    if one_count <= 1 || one_count == data.len() { return 0 }

    let mut curr_count = 0;
    for i in 0..one_count {
        if data[i] == 1 { curr_count += 1; }
    }

    let mut ans = one_count - curr_count;

    for idx in one_count..data.len() {
        if data[idx] == 1 {
            curr_count += 1;
        }
        if data[idx - one_count] == 1 {
            curr_count -= 1;
        }
        ans = ans.min(one_count - curr_count);
    }

    return ans as i32;
}
