use std::collections::{HashSet, VecDeque};

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {

    const NEI: [(i8, i8);4] = [(1,0),(-1,0),(0,1),(0,-1)];

    let base: [[i8;3];2] = [[1,2,3],[4,5,0]];
    let board = [
        [board[0][0] as i8, board[0][1] as i8, board[0][2] as i8],
        [board[1][0] as i8, board[1][1] as i8, board[1][2] as i8]];

    if base == board { return 0 }

    let mut seen: HashSet<[[i8;3];2]> = HashSet::new();
    let mut pq: VecDeque<([[i8;3];2], i32)> = VecDeque::new();
    pq.push_back((base,0));

    while let Some((state, moves)) = pq.pop_front() {

        // find the location of the '0'
        let (mut row, mut col) = (0,0);
        for r in 0..=1 {
            for c in 0..=2 {
                if state[r][c] == 0 {
                    (row, col) = (r,c)
                }
            }
        }

        // check all possible swaps
        for (rd, cd) in NEI {
            let (nr, nc) = (row as i8 + rd, col as i8 + cd);
            if nr >= 0 && nr <= 1 && nc >= 0 && nc <= 2 {

                // derive the updated state after the swap
                let mut nstate = state.clone();
                nstate[row][col] = nstate[nr as usize][nc as usize];
                nstate[nr as usize][nc as usize] = 0;

                // check it, and to the queue if it hasn't yet been seen
                if nstate == board { return moves+1 }
                if seen.insert(nstate) {
                    pq.push_back((nstate, moves+1));
                }
            }
        }
    }

    return -1;
}

#[test]
fn tests() {
    let board = vec![vec![1,2,3],vec![4,0,5]];
    assert_eq!(1, sliding_puzzle(board));

    let board = vec![vec![1,2,3],vec![5,4,0]];
    assert_eq!(-1, sliding_puzzle(board));

    let board = vec![vec![4,1,2],vec![5,0,3]];
    assert_eq!(5, sliding_puzzle(board));
}
