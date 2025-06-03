pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut state: Vec<i32> = Vec::with_capacity(ratings.len());
    state.push(1);

    for i in 1..ratings.len() {
        if ratings[i] > ratings[i-1] {
            state.push(*state.last().unwrap() + 1)
        }
        else {
            state.push(1)
        }
    }
    println!("{state:?}");

    let mut ans = 1.max(*state.last().unwrap());
    *state.last_mut().unwrap() = 1;
    for i in (0..(ratings.len()-1)).rev() {
        let mut t = 1;
        if ratings[i] > ratings[i+1] {
            t = state[i+1] + 1;
        }

        ans += t.max(state[i]);
        state[i] = t;

    }

    return ans;
}
