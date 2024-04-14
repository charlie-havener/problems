struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {val, left: None, right: None }
    }
}

use std::rc::Rc;
use std::cell::RefCell;


pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![root];

    while !stack.is_empty() {
        let node = stack.pop().unwrap();

        let node = node.unwrap();

        if node.borrow().left.is_some() 
            && node.borrow().left.as_ref().unwrap().borrow().left.is_none()
            && node.borrow().left.as_ref().unwrap().borrow().right.is_none() {
            ans += node.borrow().left.as_ref().unwrap().borrow().val;
        }

        if node.borrow().left.is_some() {
            stack.push(Some(Rc::clone(node.borrow().left.as_ref().unwrap())));
        }
        if node.borrow().right.is_some() {
            stack.push(Some(Rc::clone(node.borrow().right.as_ref().unwrap())));
        }
    }

    return ans;
}


#[test]
fn tests() {
    let n = Some(Rc::new(RefCell::new(TreeNode {val: 9, left: None, right: None})));
    let o = Some(Rc::new(RefCell::new(TreeNode {val: 15, left: None, right: None})));
    let p = Some(Rc::new(RefCell::new(TreeNode {val: 7, left: None, right: None})));

    let a = Some(Rc::new(RefCell::new(TreeNode {val: 20, left: o, right: p})));
    let root = Some(Rc::new(RefCell::new(TreeNode {val: 3, left: n, right: a})));
    assert_eq!(24, sum_of_left_leaves(root));


    let n = Some(Rc::new(RefCell::new(TreeNode {val: 9, left: None, right: None})));
    assert_eq!(0, sum_of_left_leaves(n));

}

