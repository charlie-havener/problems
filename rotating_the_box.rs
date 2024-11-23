pub fn rotate_the_box(mut b: Vec<Vec<char>>) -> Vec<Vec<char>> {
    
    let (m,n) = (b.len(), b[0].len());
    let mut ans = vec![vec!['.'; m]; n];

    for (r, row) in b.iter_mut().enumerate() {
        let (mut insert, mut search) = (n as i32 - 1, n as i32 - 1);

        while insert >= 0 && search >= 0 {

            if row[insert as usize] != '.' {
                insert -= 1;
                search = search.min(insert);
            } else if row[search as usize] == '*' {
                insert = search;
            } else if row[search as usize] == '#' {
                row[search as usize] = '.';
                row[insert as usize] = '#';
                insert -= 1;
                search -= 1;
            } else {
                search -= 1;
            }
        }
    }

    for row in 0..m {
        for col in 0..n {
            ans[col][m-row-1] = b[row][col];
        }
    }

    return ans;
}

#[test]
fn tests() {
    let b = vec![vec!['#','.','#']];
    println!("{:?}", rotate_the_box(b));

    let b = vec![vec!['#','.','*','.'],vec!['#','#','*','.']];
    println!("{:?}", rotate_the_box(b));

    let b = vec![vec!['#','#','*','.','*','.'],vec!['#','#','#','*','.','.'],vec!['#','#','#','.','#','.']];
    println!("{:?}", rotate_the_box(b));

}
