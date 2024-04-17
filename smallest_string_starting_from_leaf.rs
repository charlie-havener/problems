use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>> 
}

pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut ans: VecDeque<i32> = VecDeque::new(); 
    let mut curr: VecDeque<i32> = VecDeque::new();

    fn dfs(root: Rc<RefCell<TreeNode>>, curr: &mut VecDeque<i32>, ans: &mut VecDeque<i32>) {
        let n = root.borrow();
        curr.push_front(n.val);
        if n.left.is_none() && n.right.is_none() {
            if ans.len() == 0 || curr < ans {
                *ans = curr.clone();
            }
        }
        if let Some(l) = n.left.as_ref() {
            dfs(Rc::clone(l), curr, ans);
        }
        if let Some(r) = n.right.as_ref() {
            dfs(Rc::clone(r), curr, ans);
        }
        curr.pop_front();
    }

    dfs(root.unwrap(), &mut curr, &mut ans);
    return ans.into_iter().fold(String::new(), |mut acc, v| {
        acc.push((v as u8 + b'a') as char );
        acc
    });
}

#[test]
fn tests() {
    let a = VecDeque::from([1,2,6]);
    let b = VecDeque::from([1,2,1,1,1]);
    println!("{:?}", b<a);
}
