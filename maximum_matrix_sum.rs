pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut sum: i64 = 0;
    let mut smallest_abs: i64 = matrix[0][0].abs() as i64;
    let mut negative_count: i32 = 0;


    for row in matrix {
        for val in row {
            let abs_v = val.abs() as i64;
            sum += abs_v;
            smallest_abs = smallest_abs.min(abs_v);
            if val < 0 {
                negative_count += 1;
            }
            if val == 0 {
                negative_count -= 1;
            }
        }
    }

    println!("sum: {sum}, small: {smallest_abs}");

    if negative_count > 0 && negative_count % 2 == 1 {
        sum -= smallest_abs * 2;
    }

    return sum;
}

#[test]
fn tests() {
    let matrix = vec![vec![1,-1],vec![-1,1]];
    assert_eq!(4, max_matrix_sum(matrix));

    let matrix = vec![vec![1,2,3],vec![-1,-2,-3],vec![1,2,3]];
    assert_eq!(16, max_matrix_sum(matrix));

    let matrix = vec![vec![-1,0,-1],vec![-2,1,3],vec![3,2,2]];
    assert_eq!(15, max_matrix_sum(matrix));

    let matrix = vec![vec![-3,0,0],vec![0,0,0],vec![0,0,0]];
    assert_eq!(3, max_matrix_sum(matrix));

    let matrix = vec![vec![-3,0,0],vec![0,0,0],vec![0,3,2]];
    assert_eq!(8, max_matrix_sum(matrix));

    let matrix = vec![vec![-10000,-10000,-10000],vec![-10000,-10000,-10000],vec![-10000,-10000,-10000]];
    assert_eq!(70000, max_matrix_sum(matrix));
}
