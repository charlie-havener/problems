pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    fn helper(candidates: &Vec<i32>, target: i32, idx: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {

        if target < 0 { return; }
        if target == 0 {
            ans.push(path.clone());
            return;
        }
        for i in idx..candidates.len() {
            if i > idx && candidates[i] == candidates[i-1] {
                continue;
            }
            path.push(candidates[i]);
            helper(candidates, target - candidates[i], i + 1, path, ans);
            path.pop();
        }
    }

    candidates.sort_unstable();
    let mut ans = vec![];
    helper(&candidates, target, 0, &mut Vec::new(), &mut ans);

    return ans;
}

#[test]
fn tests() {
    let c = vec![10,1,2,7,6,1,5];
    let t = 8;
    println!("{:?}", combination_sum2(c,t));
}
