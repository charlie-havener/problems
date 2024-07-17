use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {

    if root.is_none() { return vec![] }
    if to_delete.len() == 0 { return vec![root] }

    let dummy_root = Some(Rc::new(RefCell::new(TreeNode{val: to_delete[0], left: root, right: None})));

    let to_delete = to_delete.into_iter().fold(HashSet::new(), |mut acc, v| {
        acc.insert(v);
        acc
    });

    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, to_delete: &HashSet<i32>, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> bool {
        if root.is_none() { return false }

        let root = root.as_ref().unwrap();
        let mut node = root.borrow_mut();
        
        if recurse(node.left.clone(), to_delete, ans) {
            node.left = None;
        }

        if recurse(node.right.clone(), to_delete, ans) {
            node.right = None;
        }

        if let Some(_) = to_delete.get(&node.val) {
            if node.left.is_some() {
                ans.push(node.left.clone());
            }
            if node.right.is_some() {
                ans.push(node.right.clone());
            }
            return true;
        }
        return false;
    }

    let mut ans = vec![];
    recurse(dummy_root, &to_delete, &mut ans);
    return ans;
}


#[test]
fn tests() {
    let four = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
    let five = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
    let six = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None})));
    let seven = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));

    let two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: four.clone(), right: five.clone()})));
    let three = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: six.clone(), right: seven.clone()})));

    let root = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: two.clone(), right: three.clone()})));

    let mut ans = vec![six.clone(), seven.clone(), Some(Rc::new(RefCell::new(TreeNode{val:1, left: two.clone(), right: None})))];
    ans.sort();
    let mut ret = del_nodes(root.clone(), vec![3,5]);
    ret.sort();
    assert_eq!(ans, ret);
}
