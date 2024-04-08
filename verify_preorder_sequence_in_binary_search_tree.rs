pub fn verify_preorder(preorder: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut low = i32::MIN;
    for e in preorder {
        if e < low {
            return false;
        }
        while !stack.is_empty() && e > *stack.last().unwrap() {
            low = stack.pop().unwrap();
        }
        stack.push(e);
    }
    return true;
}

#[test]
fn tests() {
    let p = vec![5,2,1,3,6];
    assert_eq!(true, verify_preorder(p));

    let p = vec![5,2,6,1,3];
    assert_eq!(false, verify_preorder(p));

    let p = vec![4,2,1,3,6,5,7];
    assert_eq!(true, verify_preorder(p));

    let p = vec![4,3,0,2,6,1,7];
    assert_eq!(false, verify_preorder(p));
}
