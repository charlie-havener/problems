use std::borrow::BorrowMut;
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

    fn to_stupid_type_def(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }
}


fn recurse(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {

    println!("pre: {:?}", preorder);
    println!("post: {:?}", postorder);
    println!("");

    if preorder.len() == 0 { return None }

    let mut root = TreeNode::new(preorder[0]);

    if preorder.len() == 1 {
        return root.to_stupid_type_def();
    }

    let mut end_idx = 0;
    while postorder[end_idx] != preorder[1] {
        end_idx += 1;
    }
    let left_len = end_idx + 1;

    root.left = recurse(&preorder[1..=left_len], &postorder[..left_len]);
    root.right = recurse(&preorder[(left_len + 1)..], &postorder[left_len..(postorder.len() -1)]);
    return root.to_stupid_type_def(); 
}



pub fn construct_from_per_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

    let mut lookups: [(usize, usize); 31] = [(0,0); 31];

    let mut dummy_head = TreeNode::new(-1);

    dummy_head.left = recurse(&preorder[..], &postorder[..]);
    
    return dummy_head.left;
}
