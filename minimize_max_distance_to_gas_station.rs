pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {

    let possible = |dist: f64| -> bool {
        (1..stations.len()).fold(0, |acc, i| { acc + ((stations[i] - stations[i-1]) as f64 / dist) as i32 }) <= k
    };

    let (mut l, mut r) = (0.0, 100_000_000.0);
    while r - l > 0.000001 {
        let m = (r + l) / 2.0;
        if possible(m) { r = m }
        else { l = m }
    }
    return l;
}

#[test]
fn tests() {
    let stations = vec![1,2,3,4,5,6,7,8,9,10];
    let k = 9;
    assert!((0.50000 - minmax_gas_dist(stations, k)).abs() < 0.000001);

    let stations = vec![23,24,36,39,46,56,57,65,84,98];
    let k = 1;
    assert!((14.00000 - minmax_gas_dist(stations, k)).abs() < 0.000001);
}
