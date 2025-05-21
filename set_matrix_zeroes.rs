pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {

    let mut r0 = false;
    for col in matrix[0].iter() {
        if *col == 0 { r0 = true }
    }

    let mut c0 = false;
    for row in matrix.iter() {
        if row[0] == 0 { c0 = true }
    }

    for row in 1..matrix.len() {
        for col in 1..matrix[0].len() {
            if matrix[row][col] == 0 {
                matrix[0][col] = 0;
                matrix[row][0] = 0;
            }
        }
    }

    for row in 1..matrix.len() {
        for col in 1..matrix[0].len() {
            if matrix[0][col] == 0 {
                matrix[row][col] = 0;
            }
            else if matrix[row][0] == 0 {
                matrix[row][col] = 0;
            }
        }
    }


    if r0 {
        for col in matrix[0].iter_mut() {
            *col = 0;
        }
    }
    if c0 {
        for row in matrix.iter_mut() {
            row[0] = 0;
        }
    }

}
