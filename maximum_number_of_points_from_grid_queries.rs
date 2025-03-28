use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {

    const NEI: [(i32, i32); 4] = [(-1,0), (0,-1), (1,0), (0,1)];
        
    let mut ans = vec![0; queries.len()];
    let mut queries = queries.into_iter().enumerate().map(|(idx, val)| (val, idx)).collect::<Vec<_>>();
    queries.sort_unstable_by_key(|k| k.0);

    let mut qptr = 0;
    let mut visited = 0;
    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    pq.push((Reverse(grid[0][0]), 0, 0));

    // once added to queue make the val negative
    // so it's easy to check if it's already in the queue
    // if it's negative then don't add it again.
    grid[0][0] *= -1;


    while let Some((Reverse(val), row, col)) = pq.pop() {
        while val >= queries[qptr].0 {
            ans[queries[qptr].1] = visited;
            qptr += 1;
            if qptr == queries.len() { return ans }
        }
        visited += 1;
        for nei in NEI {
            let nrow = row as i32 + nei.0;
            let ncol = col as i32 + nei.1;
            if nrow >= 0 && nrow < grid.len() as i32 && ncol >= 0 && ncol < grid[0].len() as i32 && grid[nrow as usize][ncol as usize] > 0 {
                pq.push((Reverse(grid[nrow as usize][ncol as usize]), nrow as usize, ncol as usize));
                grid[nrow as usize][ncol as usize] *= -1;
            }
        }
    }

    while qptr < queries.len() {
        ans[queries[qptr].1] = visited;
        qptr += 1;
    }

    return ans;
}

#[test]
fn tests() {

    let grid = vec![vec![1,2,3],vec![2,5,7],vec![3,5,1]];
    let queries = vec![5,6,2];
    assert_eq!(vec![5,8,1], max_points(grid, queries));

    let grid = vec![vec![5,2,1],vec![1,1,2]];
    let queries = vec![3];
    assert_eq!(vec![0], max_points(grid, queries));
}








