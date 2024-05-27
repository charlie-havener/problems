pub fn special_array(mut nums: Vec<i32>) -> i32 {
    let freq = nums.iter().fold(vec![0; nums.len() + 1], |mut acc, v| {
        let idx = nums.len().min(*v as usize);
        acc[idx] += 1;
        return acc;
    });

    let mut acc = 0;
    for (idx, v) in freq.iter().enumerate().rev() {
        acc += v;
        if idx == acc { return idx as i32 }
    }
    return -1;
}

#[test]
fn tests() {
    let v = vec![3,5];
    assert_eq!(2, special_array(v));
    
    let v = vec![0,0];
    assert_eq!(-1, special_array(v));

    let v = vec![0,4,3,0,4];
    assert_eq!(3, special_array(v));
}
