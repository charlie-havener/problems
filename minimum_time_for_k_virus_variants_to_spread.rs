pub fn min_daysk_variants(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut ans = i32::MAX;

    for r in 0..=100 {
        for c in 0..=100 {
            let mut p = points
                .iter()
                .fold(Vec::with_capacity(points.len()), |mut acc, p| {
                    acc.push((r - p[0]).abs() + (c - p[1]).abs());
                    acc
                });
            p.sort_unstable();
            ans = ans.min(p[k as usize - 1]);
        }
    }

    return ans;
}

#[test]
fn tests() {
    let points = vec![vec![1,1],vec![6,1]];
    let k = 2;
    assert_eq!(3, min_daysk_variants(points, k));

    let points = vec![vec![3,3],vec![1,2],vec![9,2]];
    let k = 2;
    assert_eq!(2, min_daysk_variants(points, k));

    let points = vec![vec![3,3],vec![1,2],vec![9,2]];
    let k = 3;
    assert_eq!(4, min_daysk_variants(points, k));
}
