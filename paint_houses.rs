
fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    
    let mut prev = vec![0; 3];
    let mut curr = prev.clone();

    for i in 0..costs.len() {
        for j in 0..3 {
            curr[j] = costs[i][j] + prev[(j+1)%3].min(prev[(j+2)%3]);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    return prev[0].min(prev[1].min(prev[2]));
}

#[cfg(test)]
mod test {
    use super::min_cost;

    #[test]
    fn test() {
        
        assert_eq!(8, min_cost(vec![vec![16,16,5],vec![14,3,19]]));
        assert_eq!(10, min_cost(vec![vec![17,2,17],vec![16,16,5],vec![14,3,19]]));
        assert_eq!(2, min_cost(vec![vec![7,6,2]]));
    }
}
