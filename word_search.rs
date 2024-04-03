pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    let word = word.as_bytes();

    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if board[r][c] == word[0].into() {
                visited[r][c] = true;
                if dfs(r,c,&board, &mut visited, word, 1) {
                    return true;
                }
                visited[r][c] = false;
            }
        }
    }
    return false;
}

fn dfs(r: usize, c: usize, board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, word: &[u8], curr_len: usize) -> bool {
    if curr_len == word.len() {
        return true;
    }

    // up, down, left, and right are valid search paths
    for (i,j) in [(0,1), (0,-1), (1,0), (-1,0)] {
        let new_r = j + r as i32;
        let new_c = i + c as i32;
        //println!("{new_r}, {new_c}");
        if new_r >= 0 && new_c >= 0 && new_r < board.len() as i32 && new_c < board[0].len() as i32 {
            if board[new_r as usize][new_c as usize] == word[curr_len].into() {
                if !visited[new_r as usize][new_c as usize] {
                    visited[new_r as usize][new_c as usize] = true;
                    if dfs(new_r as usize, new_c as usize, board, visited, word, curr_len + 1) {
                        return true;
                    }
                    visited[new_r as usize][new_c as usize] = false;
                }
            }
        }
    }
    return false;
}

#[test]
fn tests() {
    let v = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
    let w = String::from("ABCCED");
    assert_eq!(true, exist(v, w));

    let v = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
    let w = String::from("ABCEE");
    assert_eq!(false, exist(v, w));
    
    let v = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
    let w = String::from("SEE");
    assert_eq!(true, exist(v, w));

    let v = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
    let w = String::from("ABCB");
    assert_eq!(false, exist(v, w));
}
