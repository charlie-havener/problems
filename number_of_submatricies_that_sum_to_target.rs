use std::collections::HashMap;

fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let (r,c) = (matrix.len(), matrix[0].len());
    let mut count = 0;

    let mut sums = vec![vec![0; c+1]; r+1];
    for i in 1..r+1 {
        for j in 1..c+1 {
            sums[i][j] = matrix[i-1][j-1] + sums[i-1][j] + sums[i][j-1] - sums[i-1][j-1];
        }
    }
    println!("{:?}", sums);

    for r1 in 1..=r {
        for r2 in r1..=r {
            let mut hm = HashMap::from([(0,1)]);
            for c1 in 1..=c {
                let s = sums[r2][c1] - sums[r1-1][c1];
                count += *hm.get(&s).unwrap_or(&0);
                hm.entry(s).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::num_submatrix_sum_target;

    #[test]
    fn test() {
        assert_eq!(4, num_submatrix_sum_target(vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]], 0));
        assert_eq!(5, num_submatrix_sum_target(vec![vec![1,-1],vec![-1,1]], 0));
        assert_eq!(0, num_submatrix_sum_target(vec![vec![904]], 0));
    }

}
