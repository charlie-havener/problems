pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let mut ans: Vec<Vec<i32>> = vec![vec![0; mat2[0].len()]; mat1.len()];

    for row in 0..mat1.len() {
        for col in 0..mat2[0].len() {
            for e in 0..mat1[0].len() {
                ans[row][col] += mat1[row][e] * mat2[e][col];
            }
        }
    }

    return ans;
}
