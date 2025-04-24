pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {

    let mut r_max = 0;
    let mut r_min = image.len();
    let mut c_max = 0;
    let mut c_min = image[0].len();

    for row in 0..image.len() {
        for col in 0..image[0].len() {
            if image[row][col] == '1' {
                r_max = row.max(r_max);
                r_min = row.min(r_min);
                c_max = col.max(c_max);
                c_min = col.min(c_min);
            }
        }
    }
    
    return ((r_max - r_min + 1) * (c_max - c_min + 1)) as i32;
}

#[test]
fn tests() {
    let image = vec![vec!['0','0','1','0'],vec!['0','1','1','0'],vec!['0','1','0','0']];
    let x = 0;
    let y = 2;
    assert_eq!(6, min_area(image, x, y));

    let image = vec![vec!['1']];
    let x = 0;
    let y = 0;
    assert_eq!(1, min_area(image, x, y));
}
