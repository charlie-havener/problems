pub fn max_candies(status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {

    // 0 = empty state
    // 1 = box only
    // 2 = key only
    // 3 = visited
    let mut state = vec![0; status.len()];
    let mut ans = 0;
    let mut queue: Vec<usize> = Vec::new();

    for b in initial_boxes {
        if status[b as usize] == 1 {
            state[b as usize] = 3;
            queue.push(b as usize);
        }
        else {
            state[b as usize] = 1;
        }
    }

    while let Some(b) = queue.pop() {
        ans += candies[b];
        for contained in &contained_boxes[b] {
            if status[*contained as usize] == 1 {
                state[*contained as usize] = 3;
                queue.push(*contained as usize);
            }
            else if state[*contained as usize] == 2 {
                state[*contained as usize] = 3;
                queue.push(*contained as usize);
            }
            else {
                state[*contained as usize] = 1;
            }
        }
        for key in &keys[b] {
            if state[*key as usize] == 1 {
                state[*key as usize] = 3;
                queue.push(*key as usize);
            }
            else {
                state[*key as usize] = 2;
            }
        }
    }

    return ans;
}
