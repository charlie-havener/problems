pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![-1];
    heights.push(0);

    for (idx, &h) in heights.iter().enumerate() {
        if stack.len() == 1 {
            stack.push(idx as i32);
            continue;
        }
        while stack.len() != 1 && h < heights[*stack.last().unwrap() as usize] {
            let t = stack.pop().unwrap();
            let area = heights[t as usize] * (idx as i32 - *stack.last().unwrap() - 1);
            ans = ans.max(area);
        }
        stack.push(idx as i32);
    }
    return ans;
}

#[test]
fn tests() {
    let heights = vec![2,1,5,6,2,3];
    assert_eq!(10, largest_rectangle_area(heights));

    let heights = vec![2,4];
    assert_eq!(4, largest_rectangle_area(heights));

    let heights = vec![0];
    assert_eq!(0, largest_rectangle_area(heights));

    let heights = vec![8];
    assert_eq!(8, largest_rectangle_area(heights));

    let heights = vec![2,0,16];
    assert_eq!(16, largest_rectangle_area(heights));

    let heights = vec![4,0];
    assert_eq!(4, largest_rectangle_area(heights));

    let heights = vec![0,0,0,0,0];
    assert_eq!(0, largest_rectangle_area(heights));

    let heights = vec![1,1,1,1,2,2,1,1,1];
    assert_eq!(9, largest_rectangle_area(heights));
}
