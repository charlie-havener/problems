pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut ans = vec![vec![]];
    for n in nums {
        let e = ans.len();
        for idx in 0..e {
            let mut nxt = ans[idx].clone();
            nxt.push(n);
            ans.push(nxt);
        }
    }

    return ans;

}

#[test]
fn tests() {
    let v = vec![1,2,3];
    println!("{:?}", subsets(v));
}
