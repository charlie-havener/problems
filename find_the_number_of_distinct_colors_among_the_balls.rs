use std::collections::{HashMap, hash_map::Entry};

pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {

    let mut ans = Vec::with_capacity(queries.len());
    let mut balls: HashMap<usize, usize> = HashMap::new();
    let mut colors: HashMap<usize, usize> = HashMap::new();

    for q in queries {
        let ball_to_color = balls.entry(q[0] as usize);

        // if the ball is already colored decrement it's current color
        // the Entry of the balls current color will always be Occupied
        if let Entry::Occupied(b) = &ball_to_color {
            if let Entry::Occupied(mut c) = colors.entry(*b.get()) {
                *c.get_mut() -= 1;
                if *c.get() == 0 {
                    c.remove();
                }
            }
        }

        // set the balls color and increment that color count
        ball_to_color.insert_entry(q[1] as usize);
        colors.entry(q[1] as usize).and_modify(|count| *count += 1).or_insert(1);
        
        ans.push(colors.len() as i32);
    }
    
    return ans;
}

#[test]
fn tests() {

    let limit = 4;
    let queries = vec![vec![1,4],vec![2,5],vec![1,3],vec![3,4]];
    assert_eq!(vec![1,2,2,3], query_results(limit, queries));

    let limit = 4;
    let queries = vec![vec![0,1],vec![1,2],vec![2,2],vec![3,4],vec![4,5]];
    assert_eq!(vec![1,2,2,3,4], query_results(limit, queries));

    let limit = 1;
    let queries = vec![vec![0,999],vec![1,4654651]];
    assert_eq!(vec![1,2], query_results(limit, queries));

}
