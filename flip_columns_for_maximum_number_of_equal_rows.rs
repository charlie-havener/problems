use std::collections::HashMap;

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    
    let mut hm = HashMap::new();
    for r in &matrix {
        let mut s = String::with_capacity(matrix[0].len());
        for idx in 1..matrix[0].len() {
            if r[idx-1] == r[idx] {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        hm.entry(s).and_modify(|count| *count += 1).or_insert(1);
    }
    return *hm.values().max().unwrap();
}

#[test]
fn tests() {
    let matrix = vec![vec![0,1],vec![1,1]];
    assert_eq!(1, max_equal_rows_after_flips(matrix));

    let matrix = vec![vec![0,1],vec![1,0]];
    assert_eq!(2, max_equal_rows_after_flips(matrix));

    let matrix = vec![vec![0,0,0],vec![0,0,1],vec![1,1,0]];
    assert_eq!(2, max_equal_rows_after_flips(matrix));

    let matrix = vec![vec![1]];
    assert_eq!(1, max_equal_rows_after_flips(matrix));

    let matrix = vec![vec![1], vec![0], vec![0]];
    assert_eq!(3, max_equal_rows_after_flips(matrix));

    let matrix = vec![vec![1,0,0]];
    assert_eq!(1, max_equal_rows_after_flips(matrix));
}
