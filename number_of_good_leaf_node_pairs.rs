use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> [i32;12] {
        
        let mut curr = [0; 12];
        if root.is_none() { return curr }
        let root = root.as_ref().unwrap();
        let node = root.borrow();

        // at a leaf node
        if node.left.is_none() && node.right.is_none() {
            curr[0] = 1;
            return curr;
        }

        let left = recurse(node.left.clone(), distance);
        let right = recurse(node.right.clone(), distance);

        for i in 0..10 {
            curr[i+1] = left[i] + right[i];
        }
        curr[11] = left[11] + right[11];

        for l in 0..(distance+1) {
            for r in 0..(distance+1) {
                if distance >= 2 + l + r {
                    curr[11] += left[l as usize] * right[r as usize];
                }
            }
        }
    
        return curr;
    }

    return recurse(root.clone(), distance)[11];
}
