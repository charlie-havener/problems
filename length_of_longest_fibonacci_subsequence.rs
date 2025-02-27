use std::cmp::Ordering;

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {

    let mut dp = vec![vec![0; arr.len()]; arr.len()];
    let mut ans = 0;

    for idx in 2..arr.len() {

        let target = arr[idx];
        let mut start = 0;
        let mut end = idx-1;

        while start < end {
            let curr = arr[start] + arr[end];
            match curr.cmp(&target) {
                Ordering::Less => start += 1,
                Ordering::Greater => end -= 1,
                Ordering::Equal => {
                    dp[end][idx] = dp[start][end] + 1;
                    ans = ans.max(dp[end][idx]);
                    start += 1;
                    end -= 1;
                }
            }
        }
    }

    return if ans == 0 { 0 } else { ans + 2 };
}

#[test]
fn tests() {

    let arr = vec![1,2,3,4,5,6,7,8];
    assert_eq!(5, len_longest_fib_subseq(arr));

    let arr = vec![1,3,7,11,12,14,18];
    assert_eq!(3, len_longest_fib_subseq(arr));
}
