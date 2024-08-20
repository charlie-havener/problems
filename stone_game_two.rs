pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    0
}

#[test]
fn test() {
    let p = vec![2,7,9,4,4];
    assert_eq!(10, stone_game_ii(p));

    let p = vec![1,2,3,4,5,100];
    assert_eq!(104, stone_game_ii(p));
}
