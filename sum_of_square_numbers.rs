pub fn judge_square_sum(c: i32) -> bool {
    let max = f32::sqrt(c as f32) as i32;
    //println!("{:?}", max);
    
    for a in 0..=max {
        let b = c - a * a;
        let mut start = 0;
        let mut end = max;

        while start <= end {
            let mid = (end - start)/2 + start;
            if mid * mid == b {
                return true;
            }
            if mid*mid < b {
                start = mid + 1;
            } else {
                end = mid.wrapping_sub(1);
            }
        }
    }
    return false;
}

#[test]
fn tests() {
    let c = 5;
    assert_eq!(true, judge_square_sum(c));

    let c = 3;
    assert_eq!(false, judge_square_sum(c));

    let c = 0;
    assert_eq!(true, judge_square_sum(c));

    let c = 2147483647;
    assert_eq!(false, judge_square_sum(c));

    let c = 4;
    assert_eq!(true, judge_square_sum(c));
}
