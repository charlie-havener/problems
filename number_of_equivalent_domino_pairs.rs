pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    // since for some reason they say dominos have values up to 9...
    let mut counts = vec![vec![0; 10]; 10];
    let mut ans = 0;
    for d in dominoes {
        ans += counts[d[0].min(d[1]) as usize][d[0].max(d[1]) as usize];
        counts[d[0].min(d[1]) as usize][d[0].max(d[1]) as usize] += 1;
    }
    return ans;
}
