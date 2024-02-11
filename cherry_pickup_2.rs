pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    
    // [i][j] => [column of robot A][column of robot B]
    let mut prev = vec![vec![0; grid[0].len()]; grid[0].len()];
    let mut curr = vec![vec![0; grid[0].len()]; grid[0].len()];

    for row in (0..grid.len()).rev() {
        for a in 0..grid[0].len() {
            for b in 0..grid[0].len() {
                let p = get_max(&prev, a, b);
                curr[a][b] = match a == b {
                    true => p + grid[row][a],
                    false => p + grid[row][a] + grid[row][b]
                };
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    return prev[0][prev[0].len()-1];
}

pub fn get_max(prev: &Vec<Vec<i32>>, a: usize, b: usize) -> i32 {
    let a_low = a.checked_sub(1).unwrap_or(0);
    let a_high = (prev.len()-1).min(a+1);
    let b_low = b.checked_sub(1).unwrap_or(0);
    let b_high = (prev.len()-1).min(b+1);

    // all real values are greater than 0.
    let mut ans = -1;
    for i in a_low..=a_high {
        for j in b_low..=b_high {
            if prev[i][j] > ans { ans = prev[i][j] }
        }
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::cherry_pickup;

    #[test]
    fn test() {
        let grid = vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]];
        assert_eq!(24, cherry_pickup(grid));
        let grid = vec![vec![1,0,0,0,0,0,1],vec![2,0,0,0,0,3,0],vec![2,0,9,0,0,0,0],vec![0,3,0,5,4,0,0],vec![1,0,2,3,0,0,6]];
        assert_eq!(28, cherry_pickup(grid));
    }
}
