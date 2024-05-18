use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None, }
    }
}

pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    fn _helper(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> i32 {
        
        if let Some(r) = root {
            let x = r.borrow();
            let left = _helper(x.left.clone(), count);
            let right = _helper(x.right.clone(), count);
            
            *count += left.abs() + right.abs();
            return x.val + left + right - 1;

        }
        return 0;
    }

    let mut count = 0;
    _helper(root, &mut count);
    return count;
}

#[test]
fn test() {
    let a = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let b = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let c = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let d = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    let a = Some(Rc::new(RefCell::new(TreeNode {val: 0, left: a, right: b } )));
    let b = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: c, right: d } )));

    let root = Some(Rc::new(RefCell::new(TreeNode {val: 0, left: a, right: b } )));
    assert_eq!(6, distribute_coins(root));
}
