pub fn num_rabbits(answers: Vec<i32>) -> i32 {

    let mut ans_count = vec![0; 1_001];
    for a in answers {
        ans_count[a as usize] += 1;
    }

    let mut ans = 0;
    for (idx, val) in ans_count.iter().enumerate() {
        if *val == 0 { continue }
        let mut x = val / (idx as i32 + 1) * (idx as i32 + 1);
        if x != *val {
            x += idx as i32 + 1;
        }
        ans += x;
    }

    return ans;
}


#[test]
fn tests() {
    let answers = vec![1,1,2];
    assert_eq!(5, num_rabbits(answers));

    let answers = vec![1,1,2,2];
    assert_eq!(5, num_rabbits(answers));

    let answers = vec![1,1,2,2,2];
    assert_eq!(5, num_rabbits(answers));

    let answers = vec![1,1,2,2,2,2];
    assert_eq!(8, num_rabbits(answers));

    let answers = vec![10,10,10];
    assert_eq!(11, num_rabbits(answers));

    let answers = vec![0,0,0,0];
    assert_eq!(4, num_rabbits(answers));

    let answers = vec![1,1,1];
    assert_eq!(4, num_rabbits(answers));
}

