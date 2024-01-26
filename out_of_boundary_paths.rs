pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    
    let modulo = 1_000_000_000 + 7;

    let mut mat = vec![vec![0; n as usize]; m as usize];
    mat[start_row as usize][start_column as usize] = 1;

    let m = m as usize;
    let n = n as usize;

    let mut ans = 0;
    for mv in 0..max_move {
        for i in 0..m {
            for j in 0..n {

                // cannot move diagonally, only need to do ops on valid squares.
                if (i+j)%2 != (start_row + start_column + mv) as usize % 2 {
                    continue
                }

                if i == 0 { ans = (ans + mat[i][j]) % modulo }
                else { mat[i-1][j] = (mat[i-1][j] + mat[i][j]) % modulo }

                if i+1 >= m { ans = (ans + mat[i][j]) % modulo }
                else { mat[i+1][j] = (mat[i+1][j] + mat[i][j]) % modulo }

                if j == 0 { ans = (ans + mat[i][j]) % modulo }
                else { mat[i][j-1] = (mat[i][j-1] + mat[i][j]) % modulo }

                if j+1 >= n { ans = (ans + mat[i][j]) % modulo }
                else { mat[i][j+1] = (mat[i][j+1] + mat[i][j]) % modulo }

                // can't assign to this cell anymore. set it to 0 for use in next iteration.
                mat[i][j] = 0;
            }
        }:
    }
    return ans;
}


#[cfg(test)]
mod test {
    use super::find_paths;

    #[test]
    fn test() {
        //assert_eq!(6, find_paths(2,2,2,0,0));
        assert_eq!(12, find_paths(1, 3, 3, 0, 1));
        //assert_eq!(180, find_paths(3, 3, 5, 1, 1));
       // assert_eq!(804771989, find_paths(50,50,50,2,12));
    }
}
