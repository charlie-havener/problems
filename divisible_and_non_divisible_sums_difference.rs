pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let k = n / m;
    return ( n * (n+1) / 2 ) - ( m * k * (k + 1) );
}
