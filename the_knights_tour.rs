pub fn tour_of_knight(m: i32, n: i32, r: i32, c: i32) -> Vec<Vec<i32>> {

    fn recurse(r: i32, c: i32, board: &mut Vec<Vec<i32>>, step: i32) -> bool {

        if step as usize == board.len() * board[0].len() { return true }

        // valid space checks
        if r < 0 || r >= board.len() as i32 { return false }
        if c < 0 || c >= board[0].len() as i32 { return false }
        if board[r as usize][c as usize] != -1 { return false }


        board[r as usize][c as usize] = step;
    
        for nei in NEIGHBORS {
            let (h,v) = (nei.0, nei.1);
            if recurse(r + v, c + h, board, step+1) {
                return true;
            }
        }

        board[r as usize][c as usize] = -1;
        return false;
    }

    const NEIGHBORS: [(i32, i32); 8] = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
    let mut board: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize];
    recurse(r, c, &mut board, 0);
    return board;
}
