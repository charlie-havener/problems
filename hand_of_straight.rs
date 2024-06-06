use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {


    if hand.len() % group_size as usize != 0 {
        return false;
    }

    let mut hm = hand.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v).and_modify(|c| *c += 1).or_insert(1);
        acc
    });

    for &card in hand.iter() {
        let mut start = card;
        while hm.contains_key(&(start-1)) {
            start -= 1;
        }

        while let Some(&v) = hm.get(&start) {
            for next in 0..group_size {
                let key = start + next;
                if hm.get(&key).is_none() {
                    return false;
                }
                else if hm.get(&key).unwrap() == &v {
                    hm.remove_entry(&key);
                }
                else if hm.get(&key).unwrap() < &v {
                    return false;
                }
                else {
                    *hm.get_mut(&key).unwrap() -= v
                }
            }
            start += 1;
        }
    }
    return true;
}

#[test]
fn tests() {
    let hand = vec![1,2,3,6,2,3,4,7,8];
    let group_size = 3;
    assert_eq!(true, is_n_straight_hand(hand, group_size));

    let hand = vec![1,2,3,4,5];
    let group_size = 4;
    assert_eq!(false, is_n_straight_hand(hand, group_size));
}
