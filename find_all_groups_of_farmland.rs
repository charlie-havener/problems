pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let visited = 2;
    let mut ans = Vec::new();

    for r in 0..land.len() {
        for c in 0..land[0].len() {
            if land[r][c] == 1 {
                let ul = (r,c);
                // go right/down as far as possible
                let mut lr = (r,c);
                while lr.1 + 1 < land[0].len() && land[r][lr.1 + 1] == 1 {
                    lr.1 += 1;
                }
                while lr.0 + 1 < land.len() && land[lr.0 + 1][c] == 1 {
                    lr.0 += 1;
                }

                // add coords to ans
                ans.push(vec![ul.0 as i32, ul.1 as i32, lr.0 as i32, lr.1 as i32]);
                
                // mark grid as visited
                for i in ul.0..=lr.0 {
                    for j in ul.1..=lr.1 {
                        land[i][j] = visited;
                    }
                }
            }
        }
    }
    return ans;
}

#[test]
fn tests() {
    let land = vec![vec![1,0,0],vec![0,1,1],vec![0,1,1]];
    assert_eq!(vec![vec![0,0,0,0], vec![1,1,2,2]], find_farmland(land));

    let land = vec![vec![1,1],vec![1,1]];
    assert_eq!(vec![vec![0,0,1,1]], find_farmland(land));

    let land = vec![vec![0]];
    let a: Vec<Vec<i32>> = Vec::new();
    assert_eq!(a, find_farmland(land));
}
