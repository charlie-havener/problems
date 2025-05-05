pub fn num_tilings(n: i32) -> i32 {

    let n = n as usize;
    const MOD: i32 = 1_000_000_007;
    let mut store: [[i32; 2]; 1_001] = [[0; 2]; 1_001];

    // treat the 0 idx as if it were -1. The start is empty with a flat 0/0 edge;
    store[0] = [1,0];

    for i in 0..n {

        // store[x][0] => the 0/0 flat edge
        // store[x][1] => the 1/0 on 0/1 edge
        if i + 3 <= n {
            store[i+2][1] = (store[i+2][1] + (store[i][0] * 2).rem_euclid(MOD)).rem_euclid(MOD);
        }
        if i + 2 <= n {
            store[i+2][0] = (store[i+2][0] + store[i][0]).rem_euclid(MOD);
            store[i+1][1] = (store[i+1][1] + store[i][1]).rem_euclid(MOD);
        }
        store[i+1][0] = (store[i+1][0] + store[i][0]).rem_euclid(MOD);
        store[i+1][0] = (store[i+1][0] + store[i][1]).rem_euclid(MOD);
    }

    return store[n as usize][0] + store[n as usize][1];
}

#[test]
fn tests() {
    let n = 1;
    assert_eq!(1, num_tilings(n));

    let n = 3;
    assert_eq!(5, num_tilings(n));

    let n = 5;
    assert_eq!(24, num_tilings(n));

    let n = 1_000;
    assert_eq!(979232805, num_tilings(n));
}
