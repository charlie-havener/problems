pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut prev = Vec::with_capacity(n + 2);
    for idx in 0..n {
        if idx == 0 || idx == n - 1 { prev.push(matrix[0][idx]) }
        prev.push(matrix[0][idx]);
    }
    let mut curr = prev.clone();

    for row in 1..n {
        for col in 0..n {
            let v = matrix[row][col];
            let smallest = prev[col].min(prev[col+1].min(prev[col+2]));
            curr[col + 1] = v + smallest;
        }
        curr[0] = curr[1];
        curr[n+1] = curr[n];

        std::mem::swap(&mut prev, &mut curr);
    }
    return *prev.iter().min().unwrap() as i32;
}

#[cfg(test)]
mod test {
    use super::min_falling_path_sum;

    #[test]
    fn test() {
        assert_eq!(13, min_falling_path_sum(vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]]));
        assert_eq!(-59, min_falling_path_sum(vec![vec![-19,57],vec![-40,-5]]));
    }
}
