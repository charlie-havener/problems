pub fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
    boxes.sort_unstable_by(|a,b| b.cmp(a));
    let mut ans = 0;

    let mut left = 0;
    let mut right = warehouse.len() - 1;
    let mut idx = 0;
    
    while left<=right && idx<boxes.len() {
        if warehouse[left] >= boxes[idx] {
            ans += 1;
            left += 1;
        } else if warehouse[right] >= boxes[idx] {
            ans += 1;
            right = right.wrapping_sub(1);
        }
        idx += 1;
    }
    return ans;
}

#[test]
fn tests() {
    let b = vec![1,2,2,3,4];
    let w = vec![3,4,1,2];
    assert_eq!(4, max_boxes_in_warehouse(b, w));

    let b = vec![3,5,5,2];
    let w = vec![2,1,3,4,5];
    assert_eq!(3, max_boxes_in_warehouse(b, w));
}
