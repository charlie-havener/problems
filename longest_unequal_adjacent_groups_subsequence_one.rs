pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut ans = vec![words[0].clone()];
    let mut prev = groups[0];
    for i in 1..groups.len() {
        if groups[i] != prev {
            ans.push(words[i].clone());
            prev = groups[i];
        }
    }
    return ans;
}
