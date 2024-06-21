pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    
    let mut left = 0;
    let mut right = 0;
    
    let mut happy = 0;
    let mut window = 0;
    let mut best = 0;
    while right < grumpy.len() {

        if grumpy[right] == 0 {
            happy += customers[right];
        }

        window += customers[right] * grumpy[right];

        if right - left >= minutes as usize {
            window -= customers[left] * grumpy[left];
            left += 1;
        }

        best = best.max(window);
        right += 1;
    }

    return happy + best;

}

#[test]
fn tests() {
    let c = vec![1,0,1,2,1,1,7,5];
    let g = vec![0,1,0,1,0,1,0,1];
    let m = 3;
    assert_eq!(16, max_satisfied(c,g,m));

    let c = vec![1];
    let g = vec![0];
    let m = 1;
    assert_eq!(1, max_satisfied(c,g,m));

    let c = vec![1,0,1,2,1,1,7,5];
    let g = vec![0,1,0,1,0,1,0,1];
    let m = 8;
    assert_eq!(18, max_satisfied(c,g,m));

    let c = vec![1,0,1,2,1,1,7,5];
    let g = vec![0,0,0,0,0,0,0,0];
    let m = 8;
    assert_eq!(18, max_satisfied(c,g,m));

    let c = vec![1,0,1,2,1,1,7,5];
    let g = vec![1,1,1,1,1,1,1,1];
    let m = 2;
    assert_eq!(12, max_satisfied(c,g,m));

    let c = vec![1,0,1,2,1,1,7,5];
    let g = vec![1,1,0,0,0,0,0,0];
    let m = 2;
    assert_eq!(18, max_satisfied(c,g,m));
}
