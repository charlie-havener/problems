use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
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

fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let r = Rc::clone(root.as_ref().unwrap());
    let mut pq =  vec![r];
    let mut npq = vec![];
    let mut row_idx = -1;
    loop {
        row_idx += 1;
        let mut previous_value = match row_idx % 2 { 0 => i32::MIN, _ => i32::MAX };
        //while let Some(n) = pq.pop() {
        for idx in 0..pq.len() {
            let node = pq[idx].borrow();

            // even rows must have only odd values in strictly increasing order (l->r)
            // odd rows must have only even values in strictly decreasing order (l->r)
            if row_idx % 2 == 0 {
                if node.val % 2 == 0 { return false }
                if node.val <= previous_value { return false }
            } else {
                if node.val % 2 == 1 { return false }
                if node.val >= previous_value { return false }
            }

            // update previous and add left child then right to the next queue.
            // only if the child is not none.
            previous_value = node.val;
            if let Some(ref l) = node.left {
                npq.push(Rc::clone(l));
            }
            if let Some(ref r) = node.right {
                npq.push(Rc::clone(r));
            }
        }

        if npq.len() == 0 { return true }
        pq = npq;
        npq = vec![];
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = Some(Rc::new(RefCell::new(TreeNode::new(12))));
        let b = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        
        let c = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let d = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let e = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: a,
            right: b,
        })));
        let f = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: c,
            right: None,
        })));
        let g = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: d,
        })));

        let h = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: e,
            right: None,
        })));
        let i = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: f,
            right: g,
        })));

        let j = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: h,
            right: i,
        })));

        assert_eq!(true, is_even_odd_tree(j.clone()));
    }
}
