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
        Self { val, left: None, right: None, }
    }
}



pub fn remove_leaf_nodes(mut root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = root.as_mut() {
        let mut curr = r.borrow_mut();

        curr.left = remove_leaf_nodes(curr.left.clone(), target);
        curr.right = remove_leaf_nodes(curr.right.clone(), target);

        if curr.left.is_none() && curr.right.is_none() && curr.val == target {
            return None;
        }
    }
    return root;
}

#[test]
fn test() {
    let a = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let b = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let c = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    
    let d = Some(Rc::new(RefCell::new(TreeNode { val: 2, left: a, right: None })));
    let e = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: b, right: c })));

    let root = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: d, right: e })));
    println!("{:?}", remove_leaf_nodes(root, 2));

}
