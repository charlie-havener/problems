use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn find_distance(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {

    
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32, depth: i32) -> i32 {

        if root.is_none() { return 0 }

        let root = root.as_ref().unwrap();
        let node = root.borrow();

        if node.val == p || node.val == q {
            let left = recurse(node.left.clone(), p, q, 1);
            let right = recurse(node.right.clone(), p, q, 1);
            if left > 0 || right > 0 {
                return left.max(right);
            }
            return depth;
        }

        let left = recurse(node.left.clone(), p, q, depth+1);
        let right = recurse(node.right.clone(), p, q, depth+1);
        if left != 0 && right != 0 {
            return left + right - depth*2;
        }
        return left + right;
    }

    if p == q || root.is_none() { return 0 }
    return recurse(root, p, q, 0);
}

#[test]
fn tests() {
    let four = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
    let seven = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));

    let six = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None})));
    let two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: seven, right: four})));
    let zero = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None})));
    let eight = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: None, right: None})));

    let five = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: six, right: two})));
    let one = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: zero, right: eight})));

    let root = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: five, right: one})));


    assert_eq!(3, find_distance(root.clone(), 5, 0));
    assert_eq!(2, find_distance(root.clone(), 5, 7));
    assert_eq!(0, find_distance(root.clone(), 5, 5));
    assert_eq!(3, find_distance(root.clone(), 6, 7));
}
