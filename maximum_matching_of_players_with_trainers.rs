pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    
    players.sort_unstable();
    trainers.sort_unstable();

    let mut ans = 0;
    let mut t_idx = 0;
    let mut p_idx = 0;

    while t_idx < trainers.len() {
        if trainers[t_idx] >= players[p_idx] {
            ans += 1;
            p_idx += 1;
        }
        t_idx += 1;
        if p_idx >= players.len() {
            break;
        }
    }
    
    return ans;
}
