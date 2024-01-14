

fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut zero_losses = Vec::new();
    let mut one_losses = Vec::new();
    let mut players = vec![-1; 100001];

    for m in matches {
        let winner = m[0] as usize;
        let loser = m[1] as usize;

        if players[winner] == -1 { players[winner] = 0 }

        match players[loser] {
            -1 => players[loser] = 1,
            _ => players[loser] += 1,
        }
    }

    for idx in 0..players.len() {
        match players[idx] {
            0 => zero_losses.push(idx as i32),
            1 => one_losses.push(idx as i32),
            _ => (),
        }
    }

    return vec![zero_losses, one_losses];
}

#[cfg(test)]
mod test {
    use super::find_winners;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1,2,10],vec![4,5,7,8]], find_winners(vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]]));
        assert_eq!(vec![vec![1,2,5,6],vec![]], find_winners(vec![vec![2,3],vec![1,3],vec![5,4],vec![6,4]]));
    }
}
