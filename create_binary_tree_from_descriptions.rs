use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
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


use std::collections::HashMap;

#[derive(Debug)]
struct HMValue {
    left: Option<usize>,
    right: Option<usize>,
    is_child: bool,
}

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {

    let hm: HashMap<i32, HMValue> = descriptions.iter().enumerate().fold(HashMap::new(), |mut acc, (idx, d)| {

        let (parent, child, is_left) = (d[0], d[1], d[2]);

        // update parent with child
        if is_left == 0 {
            acc.entry(parent).and_modify(|v| v.right = Some(idx)).or_insert(HMValue{left: None, right: Some(idx), is_child: false});
        } else {
            acc.entry(parent).and_modify(|v| v.left = Some(idx)).or_insert(HMValue{left: Some(idx), right: None, is_child: false});
        }

        // mark child as a child
        acc.entry(child).and_modify(|v| v.is_child = true).or_insert(HMValue{left: None, right: None, is_child: true});


        return acc;

    });

    // determine the root and initialize the queue
    let mut root = None;
    let mut pq = vec![];
    for (k, v) in hm.iter() {
        if !v.is_child {
            root = Some(Rc::new(RefCell::new(TreeNode::new(*k))));
            


            pq.push(Rc::clone(root.as_ref().unwrap()));
            break;
        }
    }

    // build the tree
    while let Some(curr) = pq.pop() {
        let mut node = curr.borrow_mut();
        
        if let Some(desc_idx) = hm[&node.val].left {
            let new_node = TreeNode::new(descriptions[desc_idx][1]);
            node.left = Some(Rc::new(RefCell::new(new_node)));
            pq.push(node.left.as_ref().unwrap().clone());
        }
        if let Some(desc_idx) = hm[&node.val].right {
            let new_node = TreeNode::new(descriptions[desc_idx][1]);
            node.right = Some(Rc::new(RefCell::new(new_node)));
            pq.push(node.right.as_ref().unwrap().clone());
        }
    }

    return root;

}

#[test]
fn test() {
    let d = vec![vec![20,15,1],vec![20,17,0],vec![50,20,1],vec![50,80,0],vec![80,19,1]];
    println!("{:?}", create_binary_tree(d));
}
