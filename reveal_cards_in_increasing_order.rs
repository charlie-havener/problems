use std::collections::VecDeque;

pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    deck.sort_unstable();

    let mut d = VecDeque::from_iter(0..deck.len());

    let mut ans = vec![0; deck.len()];
    let mut s = deck.into_iter();

    while !d.is_empty() {
        let i = d.pop_front().unwrap();
        ans[i] = s.next().unwrap();
        if !d.is_empty() {
            let n = d.pop_front().unwrap();
            d.push_back(n);
        }
    }
    
    println!("{:?}", ans);
    return ans;
}

#[test]
fn tests() {
    let v = vec![17,13,11,2,3,5,7];
    assert_eq!(vec![2,13,3,11,5,17,7], deck_revealed_increasing(v));

    let v = vec![1,1000];
    assert_eq!(vec![1,1000], deck_revealed_increasing(v));

    let v = vec![17,13,11,2,3,5,7,6];
    assert_eq!(vec![2,7,3,13,5,11,6,17], deck_revealed_increasing(v));
}
