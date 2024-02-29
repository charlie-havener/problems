use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}


fn equal_to_descendants(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    
    fn dfs(root: Rc<RefCell<TreeNode>>, count: &mut i32) -> i32 {
        let node = root.borrow();
        let (mut left_sum, mut right_sum) = (0,0);

        if let Some(ref l) = node.left {
            left_sum += dfs(Rc::clone(l), count);
        }
        if let Some(ref r) = node.right {
            right_sum += dfs(Rc::clone(r), count);
        }

        if node.val == left_sum + right_sum {
            *count += 1;
        }
        return node.val + left_sum + right_sum;
    }

    let mut count = 0;
    let _ = dfs(Rc::clone(root.as_ref().unwrap()), &mut count);
    return count;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let b = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let c = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let d = Some(Rc::new(RefCell::new(TreeNode {
            val: 3, left: a, right: b,
        })));
        let e = Some(Rc::new(RefCell::new(TreeNode {
            val: 10, left: d, right: c,
        })));
        assert_eq!(2, equal_to_descendants(e));
    }

    #[test]
    fn test2() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let b = Some(Rc::new(RefCell::new(TreeNode {
            val: 3, left: a, right: None,
        })));
        let c = Some(Rc::new(RefCell::new(TreeNode {
            val: 2, left: b, right: None,
        })));
        assert_eq!(0, equal_to_descendants(c));
    }

    #[test]
    fn test3() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(1, equal_to_descendants(a));
    }

    #[test]
    fn test4() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let b = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let d = Some(Rc::new(RefCell::new(TreeNode {
            val: 3, left: a, right: b,
        })));
        let e = Some(Rc::new(RefCell::new(TreeNode {
            val: 10, left: d, right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })));
        assert_eq!(2, equal_to_descendants(e));
    }
}


