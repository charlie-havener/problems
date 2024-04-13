pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let mut prev = vec![0; matrix[0].len() + 1];
    let mut curr = prev.clone();
    let mut ans = 0;

    for row in 0..matrix.len() {
        let mut stack = vec![-1];
        for col in 0..=matrix[0].len() {
            
            if col == matrix[0].len() {
                curr[col] = 0;
            } else {
                curr[col] = match matrix[row][col] {
                    '0' => 0,
                    '1' => prev[col] + 1,
                    _ => unreachable!(),
                }
            }

            if stack.len() == 1 {
                stack.push(col as i32);
                continue;
            }

            while stack.len() != 1 && curr[col] < curr[*stack.last().unwrap() as usize] {
                let t = stack.pop().unwrap();
                let area = curr[t as usize] * (col as i32 - *stack.last().unwrap() - 1);
                ans = ans.max(area);
            }

            stack.push(col as i32);

        }

        println!("prev: {prev:?}, curr: {curr:?}");
        std::mem::swap(&mut prev, &mut curr);
    }

    return ans;
}

#[test]
fn tests() {
    let matrix = vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']];
    assert_eq!(6, maximal_rectangle(matrix));

    let matrix = vec![vec!['0']];
    assert_eq!(0, maximal_rectangle(matrix));

    let matrix = vec![vec!['1']];
    assert_eq!(1, maximal_rectangle(matrix));
}
