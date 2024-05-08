pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut score = score.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    score.sort_by_key(|(_,v)| *v * -1);

    let mut ans = vec![String::new(); score.len()];
    for (idx, (i, _)) in score.into_iter().enumerate() {
        let t = &mut ans[i];
        match idx {
            0 => *t = String::from("Gold Medal"),
            1 => *t = String::from("Silver Medal"),
            2 => *t = String::from("Bronze Medal"),
            q => *t = (q+1).to_string(),
        }
    }
    return ans;
}

#[test]
fn tests() {
    let score = vec![5,4,3,2,1];
    find_relative_ranks(score);

    let score = vec![10,3,8,9,4];
    find_relative_ranks(score);
}
