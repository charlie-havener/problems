use std::collections::HashMap;

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut hm = HashMap::from([(0,-1)]);
    let mut ans = 0;
    
    let mut sum = 0;
    for (idx, &val) in nums.iter().enumerate() {
        match val {
            0 => sum -= 1,
            1 => sum += 1,
            _ => unreachable!(),
        }
        match hm.get(&sum) {
            Some(&i) => ans = ans.max(idx as i32 - i),
            None => _ = hm.insert(sum, idx as i32),
        }
    }
    return ans;
}


#[test]
fn tests() {
    let v = vec![0,1];
    assert_eq!(2, find_max_length(v));

    let v = vec![0,1,0];
    assert_eq!(2, find_max_length(v));

    let v = vec![0,0,0,1,1,0,1,1,1,0];
    assert_eq!(10, find_max_length(v));

    let v = vec![0,0,1,1,0,1,1,1,0];
    assert_eq!(6, find_max_length(v));
    
    let v = vec![0];
    assert_eq!(0, find_max_length(v));

    let v = vec![1];
    assert_eq!(0, find_max_length(v));

    let v = vec![1,1,1,1,1,1];
    assert_eq!(0, find_max_length(v));

    let v = vec![0,0,0,0,0,0];
    assert_eq!(0, find_max_length(v));
    
    let v = vec![0,0,1,0,0,0];
    assert_eq!(2, find_max_length(v));
}
