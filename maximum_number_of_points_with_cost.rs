pub fn max_points(mut points: Vec<Vec<i32>>) -> i64 {
    
    let rows = points.len();
    let cols = points[0].len();

    let mut prev = points[0].iter().fold(Vec::new(), |mut acc, v| {acc.push(*v as i64); acc });
    //let mut prev = vec![0; cols];
    let mut curr = vec![0; cols];
    
    for r in 1..rows {
        // from the left
        let mut tot = 0; 
        for c in 0..cols {
            tot = (tot - 1).max(prev[c]);
            curr[c] = tot;
        }

        // from the right
        tot = 0;
        for c in (0..cols).rev() {
            tot = (tot-1).max(prev[c]);
            curr[c] = tot.max(curr[c]) + points[r][c] as i64;
        }

        std::mem::swap(&mut prev, &mut curr);
    }
    return *prev.iter().max().unwrap();
}

#[test]
fn tests() {
    let p = vec![vec![2,7,4],vec![1,5,1],vec![3,1,1]];
    assert_eq!(14, max_points(p));

    let p = vec![vec![1,2,3],vec![1,5,1],vec![3,1,1]];
    assert_eq!(9, max_points(p));

    let p = vec![vec![1,5],vec![2,3],vec![4,2]];
    assert_eq!(11, max_points(p));
}

