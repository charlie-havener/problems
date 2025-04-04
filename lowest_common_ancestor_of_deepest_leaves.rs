use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

struct EPath {
    nodes: Vec<Rc<RefCell<TreeNode>>>,
    heights: Vec<i32>,
}

impl EPath {
    fn generate(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = Vec::new();
        let mut heights = Vec::new();
        Self::_dfs(root.unwrap(), &mut nodes, &mut heights);
        return Self {nodes, heights};
    }

    fn _dfs(root: Rc<RefCell<TreeNode>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>, heights: &mut Vec<i32>) {
        heights.push(1 + *heights.last().unwrap_or(&-1));
        nodes.push(Rc::clone(&root));

        if let Some(l) = root.borrow().left.as_ref() {
            Self::_dfs(Rc::clone(l), nodes, heights);
            heights.push(*heights.last().unwrap() - 1);
            nodes.push(Rc::clone(&root));
        }

        if let Some(r) = root.borrow().right.as_ref() {
            Self::_dfs(Rc::clone(r), nodes, heights);
            heights.push(*heights.last().unwrap() - 1);
            nodes.push(Rc::clone(&root));
        }
    }

    fn find_target(&self) -> Option<Rc<RefCell<TreeNode>>> {
        let mut curr_deepest = 0;
        let mut lca = 0;
        let mut potential_lca = 0;


        for idx in 0..self.nodes.len() {

            // a new deepest node => reset
            if self.heights[idx] > self.heights[curr_deepest] {
                curr_deepest = idx;
                lca = idx;
                potential_lca = idx;
            }

            // a node of the same depth found
            else if self.heights[idx] == self.heights[curr_deepest] {
                lca = potential_lca;
                potential_lca = lca;
            }

            // a node that could be an lca if another curr_deepest depth is found
            else {
                if self.heights[idx] < self.heights[potential_lca] {
                    potential_lca = idx;
                }
            }

        }
        
        return Some(Rc::clone(&self.nodes[lca]));

    }
}


pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let x = EPath::generate(root);
    println!("{:?}", x.heights);
    return None;
}
