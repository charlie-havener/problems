use std::collections::{HashSet, HashMap};

fn dfs(coord: (i32, i32), rows: &HashMap<i32, Vec<i32>>, cols: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<(i32, i32)>) -> i32 {

    println!("  dfs @ {:?}", coord);
    let r = coord.0;
    let c = coord.1;
    visited.insert(coord);

    let mut count = 1;
    if let Some(same_row) = rows.get(&r) {
        for p in same_row {
            let cell = (r,*p);
            if !visited.contains(&cell) {
                visited.insert(cell);
                count += dfs(cell, rows, cols, visited);
            }
        }
    }

    if let Some(same_col) = cols.get(&c) {
        for p in same_col {
            let cell = (*p, c);
            if !visited.contains(&cell) {
                visited.insert(cell);
                count += dfs(cell, rows, cols, visited);
            }
        }
    }
    
    return count;
}


pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {

    let mut rows: HashMap<i32,Vec<i32>> = HashMap::new();
    let mut cols: HashMap<i32,Vec<i32>> = HashMap::new();
    for coord in stones.iter() {
        let r = coord[0];
        let c = coord[1];
        rows.entry(r).and_modify(|v| v.push(c)).or_insert(vec![c]);
        cols.entry(c).and_modify(|v| v.push(r)).or_insert(vec![r]);
    }

    
    let mut ans = 0;
    let mut visited: HashSet<(i32,i32)> = HashSet::with_capacity(stones.len());
    for coord in stones.iter() {
        let coord = (coord[0], coord[1]);
        if !visited.contains(&coord) {
            println!("new dfs @ {:?}", coord);
            let size = dfs(coord, &rows, &cols, &mut visited);
            ans += size - 1;
        }
    }
    
    return ans;
}

#[test]
fn tests() {
    //let s = vec![vec![0,0],vec![0,1],vec![1,0],vec![1,2],vec![2,1],vec![2,2]];
    //assert_eq!(5, remove_stones(s));

    //let s = vec![vec![0,0],vec![0,2],vec![1,1],vec![2,0],vec![2,2]];
    //assert_eq!(3, remove_stones(s));

    //let s = vec![vec![0,0]];
    //assert_eq!(0, remove_stones(s));

    let s = vec![vec![0,1],vec![1,2],vec![1,3],vec![3,3],vec![2,3],vec![0,2]];
    assert_eq!(5, remove_stones(s));
}
