pub fn longest_square_streak(nums: Vec<i32>) -> i32 {

    let mut vs: Vec<bool> = vec![false; 100_001];
    for v in &nums {
        vs[*v as usize] = true;
    }
    
    let mut ans = 0;
    for v in &nums {
        let mut count = 0;
        let mut v = v*v;
        while v < 317 && vs[v as usize] {
            println!("{v}");
            count += 1;
            v *= v;
        }
        ans = ans.max(count);
    }
    if ans < 2 { return -1 }
    return ans;
}

#[test]
fn tests() {
    let nums = vec![4,3,6,16,8,2];
    assert_eq!(3, longest_square_streak(nums));

    let nums = vec![2,3,5,6,7];
    assert_eq!(-1, longest_square_streak(nums));
}
