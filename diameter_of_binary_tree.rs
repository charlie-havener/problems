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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn helper_diameter(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    // tuple is the best ans so far, and the largest depth from node.
    
    let n = root.borrow();

    match (n.left.as_ref(), n.right.as_ref()) {
        (Some(l), Some(r)) => {
            let (lans, ldepth) = helper_diameter(Rc::clone(&l));
            let (rans, rdepth) = helper_diameter(Rc::clone(&r));
            let ans = lans.max(rans.max(ldepth + rdepth + 2));
            let depth = ldepth.max(rdepth) + 1;
            return (ans, depth);
        }
        (Some(x), None) | (None, Some(x)) => {
            let (ans, depth) = helper_diameter(Rc::clone(&x));
            let ans = ans.max(depth + 1);
            let depth = depth + 1;
            return (ans, depth);
        },
        (None, None) => return (0,0),
    }
}


fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    match root {
        None => 0,
        Some(x) => helper_diameter(x).0,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(0, diameter_of_binary_tree(t.clone()));

        let s = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let r = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: s,
            right: t,
        })));
        assert_eq!(2, diameter_of_binary_tree(r.clone()));

        let q = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: r,
            right: q,
        })));
        assert_eq!(3, diameter_of_binary_tree(p.clone()));
    }
}
