pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cur_d = 0;
    let movements = [[0,1],[1,0],[0,-1],[-1,0]];
    let mut ans = vec![vec![-1; n as usize]; m as usize];

    let mut h = &head;
    while let Some(node) = h {
        ans[i as usize][j as usize] = node.val;
        let next_i = i + movements[cur_d][0];
        let next_j = j + movements[cur_d][1];

        if next_i < 0 || next_j < 0 || next_i >= m || next_j >= n || ans[next_i as usize][next_j as usize] != -1 {
            cur_d = (cur_d + 1).rem_euclid(4);
        }
        i += movements[cur_d][0];
        j += movements[cur_d][1];
        h = &node.next;
    }

    return ans;
}
