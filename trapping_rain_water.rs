pub fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut max_height = 0;
    let mut store = vec![0; height.len()];
    
    for idx in 0..height.len() {
        store[idx] = max_height;
        max_height = max_height.max(height[idx]);
    }

    max_height = 0;
    for idx in (0..height.len()).rev() {
        ans += (store[idx].min(max_height) - height[idx]).max(0);
        max_height = max_height.max(height[idx]);
    }

    return ans;
}


#[test]
fn test() {
    let heights = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    assert_eq!(6, trap(heights));
    
    let heights = vec![4,2,0,3,2,5];
    assert_eq!(9, trap(heights));

    let heights = vec![0,0,1,0,0];
    assert_eq!(0, trap(heights));

    let heights = vec![10,0,10];
    assert_eq!(10, trap(heights));

    let heights = vec![10,0,0];
    assert_eq!(0, trap(heights));
}
