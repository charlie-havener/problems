use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    area: Vec<usize>,
    large: usize,
}

impl UnionFind {
    
    fn new(size: usize) -> Self {
        return Self {
            parents: (0..size).collect::<Vec<_>>(),
            ranks: vec![0; size],
            area: vec![0; size],
            large: 0,
        };
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x { return x }
        self.parents[x] = self.find(self.parents[x]);
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) -> () {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return }
        match self.ranks[px].cmp(&self.ranks[py]) {
            Ordering::Less => {
                self.parents[px] = py;
                self.area[py] += self.area[px];
                self.large = self.large.max(self.area[py]);
            },
            Ordering::Greater => {
                self.parents[py] = px;
                self.area[px] += self.area[py];
                self.large = self.large.max(self.area[px]);
            },
            Ordering::Equal => {
                self.parents[px] = py;
                self.ranks[py] += 1;
                self.area[py] += self.area[px];
                self.large = self.large.max(self.area[py]);
            },
        }
    }
}

pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    
    let coord_to_index = |row: usize, col: usize| -> usize {
        row * grid[0].len() + col
    };
    
    let is_valid_coord = |row: i32, col: i32| -> bool {
        if row >= 0 && row < grid.len() as i32
        && col >= 0 && col < grid[0].len() as i32
        && grid[row as usize][col as usize] == 1 {
            return true;
        }
        return false;
    };

    // traversing rows down, cols right so only need to check left and up (the first 2)
    const NEI: [(i32, i32); 4] = [(-1,0),(0,-1),(1,0),(0,1)];


    let mut uf = UnionFind::new(grid.len() *grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 { continue }
            let idx = coord_to_index(row, col);
            uf.area[idx] = 1;

            for nei in &NEI[0..2] {
                let nr = row as i32 + nei.0;
                let nc = col as i32 + nei.1;
                if !is_valid_coord(nr, nc) { continue }
                let nidx = coord_to_index(nr as usize, nc as usize);
                uf.union(idx, nidx);
            }
        }
    }
    
    // traverse all cells again and check what happens if it were changed to a 1
    let mut ans = uf.large;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 0 { continue }
            let idx = coord_to_index(row, col);
            let mut neighbors = vec![idx;4];
            for (i, nei) in NEI.iter().enumerate() {
                let nr = row as i32 + nei.0;
                let nc = col as i32 + nei.1;
                if !is_valid_coord(nr, nc) { continue }
                let nidx = coord_to_index(nr as usize, nc as usize);
                neighbors[i] = uf.find(nidx);
            }
            neighbors.sort_unstable();
            neighbors.dedup();
            ans = ans.max(1 + neighbors.iter().map(|i| uf.area[*i]).sum::<usize>());
        }
    }

    return ans as i32;

}

#[test]
fn tests() {
    let grid = vec![vec![0,1,0],vec![1,0,1],vec![0,1,0]];
    assert_eq!(5, largest_island(grid));

    let grid = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    assert_eq!(9, largest_island(grid));

    let grid = vec![vec![1,1,1],vec![1,0,0],vec![1,0,0]];
    assert_eq!(6, largest_island(grid));

    let grid = vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
    assert_eq!(1, largest_island(grid));

    let grid = vec![vec![1,0,0],vec![0,0,0],vec![0,0,1]];
    assert_eq!(2, largest_island(grid));
}















