
use std::cmp::Ordering;

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    tokens.sort_unstable();

    let mut ans = 0;
    let mut score = 0;
    let mut ptr1 = 1;
    let mut ptr2 = tokens.len();

    while ptr1 <= ptr2 {
        match tokens[ptr1 - 1].cmp(&power) {
            Ordering::Less | Ordering::Equal => {
                power -= tokens[ptr1 - 1];
                ptr1 += 1;
                score += 1;
            },
            Ordering::Greater if score > 0 => {
                power += tokens[ptr2 - 1];
                ptr2 -= 1;
                score -= 1;
            },
            _ => return ans,
        }
        ans = ans.max(score);
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, bag_of_tokens_score(vec![100], 50));
        assert_eq!(1, bag_of_tokens_score(vec![200,100], 150));
        assert_eq!(2, bag_of_tokens_score(vec![100,200,300,400], 200));
        assert_eq!(10, bag_of_tokens_score(vec![10,10,10,10,10,10,10,10,10,10,10,110], 10));
        assert_eq!(10, bag_of_tokens_score(vec![10,10,10,10,10,10,10,10,10,10], 100));
        assert_eq!(0, bag_of_tokens_score(vec![71,55,82], 54));
    }
}
