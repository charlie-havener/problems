use std::cmp::Ordering;

#[derive(Debug)]
struct UnionFind {
    parents: Vec<Vec<(usize,usize)>>,
    ranks: Vec<Vec<usize>>,
    count: usize,
}

impl UnionFind {
    fn new(r: usize, c: usize) -> Self {

        let mut parents = vec![vec![(0,0);c]; r];
        let ranks = vec![vec![0;c];r];
        for row in 0..r {
            for col in 0..c {
                parents[row][col] = (row,col);
            }
        }

        Self {parents, ranks, count: 0}
    }

    fn find(&mut self, coord: (usize, usize)) -> (usize, usize) {
        if self.parents[coord.0][coord.1] == coord { return coord }
        self.parents[coord.0][coord.1] = self.find(self.parents[coord.0][coord.1]);
        self.parents[coord.0][coord.1]
    }

    fn union(&mut self, coord1: (usize, usize), coord2: (usize, usize)) {
        let p1 = self.find(coord1);
        let p2 = self.find(coord2);

        // they are already unioned
        if p1 == p2 { return }

        self.count -= 1;
        match self.ranks[p1.0][p1.1].cmp(&self.ranks[p2.0][p2.1]) {
            Ordering::Less => self.parents[p1.0][p1.1] = p2,
            Ordering::Greater => self.parents[p2.0][p2.1] = p1,
            Ordering::Equal => {
                self.parents[p2.0][p2.1] = p1;
                self.ranks[p1.0][p1.1] += 1;
            },
        }
    }
}

fn is_valid(r: i32, c: i32, grid: &Vec<Vec<i32>>) -> bool {
    // cell is valid if it is unvised
    // and is within the bounds of grid
    if r < 0 || r >= grid.len() as i32 { return false }
    if c < 0 || c >= grid[0].len() as i32 { return false }
    if grid[r as usize][c as usize] != 1 { return false }
    return true;
}

fn dfs(coord: (usize, usize), grid: &mut Vec<Vec<i32>>, uf: &mut UnionFind) {
    
    let r = coord.0 as i32;
    let c = coord.1 as i32;

    // mark this node as visited and add an island to count
    grid[coord.0][coord.1] = 2;
    uf.count += 1;

    for dir in [(-1,0), (1,0), (0,-1), (0,1)] {
        let (nr, nc) = (r + dir.0, c + dir.1);
        if is_valid(nr, nc, grid) {
            uf.union((r as usize, c as usize), (nr as usize ,nc as usize));
            dfs((nr as usize, nc as usize), grid, uf);
        }
    }
}

fn dfs2(coord: (usize, usize), grid: &mut Vec<Vec<i32>>, uf: &mut UnionFind, parent: (usize, usize), valid: &mut bool) {

    let r = coord.0 as i32;
    let c = coord.1 as i32;

    // mark this node as visited and add an island to count
    grid[coord.0][coord.1] = 2;

    for dir in [(-1,0), (1,0), (0,-1), (0,1)] {
        let (nr, nc) = (r + dir.0, c + dir.1);
        if is_valid(nr, nc, grid) {
            if uf.find((nr as usize,nc as usize)) != parent {
                *valid = false;
            }
            dfs2((nr as usize, nc as usize), grid, uf, parent, valid);
        }
    }
}

pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {

    let m = grid1.len();
    let n = grid1[0].len();
    let mut uf = UnionFind::new(m,n);

    // since grids only contain 0 and 1 and we don't traverse from water
    // we can set visited land to 2 in order to mark nodes visited
    // 0 => water
    // 1 => land
    // 2+ => visited land
    for row in 0..m {
        for col in 0..n {
            if grid1[row][col] == 1 {
                dfs((row, col), &mut grid1, &mut uf);
            }
        }
    }
    
    // now walk through grid2.
    // if new land then store the parent of coord from uf
    // dfs from the coord and check that the parent is th same
    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if grid2[r][c] == 1 && grid1[r][c] >= 1 {
                let p = uf.find((r,c));
                let mut valid = true;
                dfs2((r,c), &mut grid2, &mut uf, p, &mut valid);
                if valid { ans += 1 }
            }
        }
    }
    return ans;
}

#[test]
fn tests() {
        
    let grid1 = vec![vec![1,1,1,0,0],vec![0,1,1,1,1],vec![0,0,0,0,0],vec![1,0,0,0,0],vec![1,1,0,1,1]];
    let grid2 = vec![vec![1,1,1,0,0],vec![0,0,1,1,1],vec![0,1,0,0,0],vec![1,0,1,1,0],vec![0,1,0,1,0]];
    assert_eq!(3, count_sub_islands(grid1, grid2));

    let grid1 = vec![vec![1,0,1,0,1],vec![1,1,1,1,1],vec![0,0,0,0,0],vec![1,1,1,1,1],vec![1,0,1,0,1]];
    let grid2 = vec![vec![0,0,0,0,0],vec![1,1,1,1,1],vec![0,1,0,1,0],vec![0,1,0,1,0],vec![1,0,0,0,1]];
    assert_eq!(2, count_sub_islands(grid1, grid2));
}
