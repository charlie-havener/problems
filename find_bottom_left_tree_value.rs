use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}


fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    let mut ans = root.as_ref().unwrap().borrow().val;
    let mut pq = vec![Rc::clone(root.as_ref().unwrap())];
    let mut npq = vec![];
    
    while pq.len() > 0 {
        ans = pq[0].borrow().val;
        for idx in 0..pq.len() {
            let node = pq[idx].borrow();
            if let Some(ref l) = node.left {
                npq.push(Rc::clone(l));
            }
            if let Some(ref r) = node.right {
                npq.push(Rc::clone(r));
            }
        }
        pq = npq;
        npq = vec![];
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let o = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let p = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: o,
            right: p,
        })));
        assert_eq!(1, find_bottom_left_value(t.clone()));

        let o = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let l = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: o, right: None})));
        let p = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let f = Some(Rc::new(RefCell::new(TreeNode {val: 5, left: p, right: None})));
        let r = Some(Rc::new(RefCell::new(TreeNode {val: 3, left: f, right: q})));
        let t = Some(Rc::new(RefCell::new(TreeNode {val: 1, left: l, right: r})));
        assert_eq!(7, find_bottom_left_value(t.clone()));

    }
}
