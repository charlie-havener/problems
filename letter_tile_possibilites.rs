use std::collections::HashSet;

const P1: i64 = 29;
const P2: i64 = 1_000_000_007;

fn explore(hash: i64, letters: &mut [usize; 26], seen: &mut HashSet<i64>) {

    for idx in 0..letters.len() {
        if letters[idx] == 0 { continue; }

        // use the letter
        letters[idx] -= 1;
        let new_hash = (P1 * hash + (idx as i64 + 1)).rem_euclid(P2);

        // if it's a new hash
        if seen.insert(new_hash) {
            explore(new_hash, letters, seen);
        }

        // readd the letter
        letters[idx] += 1;
    }
}


pub fn num_tile_possibilities(tiles: String) -> i32 {

    let mut letters = tiles.chars().fold([0_usize; 26], |mut acc, l| {
        acc[(l as u8 - b'A') as usize] += 1;
        acc
    });

    let mut seen: HashSet<i64> = HashSet::new();

    explore(1, &mut letters, &mut seen);

    return seen.len() as i32;
}

#[test]
fn tests() {
    let tiles = String::from("AAB");
    assert_eq!(8, num_tile_possibilities(tiles));

    let tiles = String::from("AAABBC");
    assert_eq!(188, num_tile_possibilities(tiles));

    let tiles = String::from("V");
    assert_eq!(1, num_tile_possibilities(tiles));

    let tiles = String::from("AAAAAAA");
    assert_eq!(7, num_tile_possibilities(tiles));

    let tiles = String::from("AAAAAAB");
    assert_eq!(34, num_tile_possibilities(tiles));

    let tiles = String::from("ABCDEFG");
    assert_eq!(13699, num_tile_possibilities(tiles));
}
