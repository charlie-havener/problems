pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.clone();
    sorted.sort_unstable();
    let mut count = 0;
    for i in 0..heights.len() {
        if heights[i] != sorted[i] {
            count += 1;
        }
    }
    return count;
}

#[test]
fn tests() {
    let v = vec![1,1,4,2,1,3];
    assert_eq!(3, height_checker(v));
    let v = vec![5,1,2,3,4];
    assert_eq!(5, height_checker(v));
    let v = vec![1,2,3,4,5];
    assert_eq!(0, height_checker(v));
}
