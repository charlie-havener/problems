pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut iv = intervals.into_iter();
    while let Some(x) = iv.next() {
        if x[0] >= to_be_removed[1] || x[1] <= to_be_removed[0] {
            ans.push(x);
        } else {
            if x[0] < to_be_removed[0] {
                ans.push(vec![x[0], to_be_removed[0]]);
            }
            if x[1] > to_be_removed[1] {
                ans.push(vec![to_be_removed[1], x[1]]);
            }
        }
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let intervals = vec![vec![0,2],vec![3,4],vec![5,7]];
        let remove = vec![1,6];
        let ans = vec![vec![0,1],vec![6,7]];
        assert_eq!(ans, remove_interval(intervals, remove));

        let intervals = vec![vec![0,5]];
        let remove = vec![2,3];
        let ans = vec![vec![0,2],vec![3,5]];
        assert_eq!(ans, remove_interval(intervals, remove));

        let intervals = vec![vec![-5,-4],vec![-3,-2],vec![1,2],vec![3,5],vec![8,9]];
        let remove = vec![-1,4];
        let ans = vec![vec![-5,-4],vec![-3,-2],vec![4,5],vec![8,9]];
        assert_eq!(ans, remove_interval(intervals, remove));
    }
}
