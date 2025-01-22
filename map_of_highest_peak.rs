use std::collections::VecDeque;

pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    const NEI: [(i32,i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];
    let row_count = is_water.len() as i32;
    let col_count = is_water[0].len() as i32;

    let mut vd: VecDeque<(i32, i32)> = VecDeque::new();

    for row in 0..row_count {
        for col in 0..col_count {
            if is_water[row as usize][col as usize] == 1 {
                vd.push_back((row, col));
                is_water[row as usize][col as usize] = 0;
            } else {
                is_water[row as usize][col as usize] = -1;
            }
        }
    }

    while vd.len() > 0 {
        let count = vd.len();
        for _ in 0..count {
            let Some((row, col)) = vd.pop_front() else { panic!("ahhh") };
            for nei in NEI {
                let n_row = row as i32 + nei.0;
                let n_col = col as i32 + nei.1;
                if n_row >= 0 && n_row < row_count && n_col >= 0 && n_col < col_count && is_water[n_row as usize][n_col as usize] == -1 {
                    is_water[n_row as usize][n_col as usize] = is_water[row as usize][col as usize] + 1;
                    vd.push_back((n_row, n_col));
                }
            }
        }
    }

    return is_water;
}


#[test]
fn tests() {
    let is_water = vec![vec![0,1],vec![0,0]];
    println!("{:?}", highest_peak(is_water));

    let is_water = vec![vec![0,0,1],vec![1,0,0],vec![0,0,0]];
    println!("{:?}", highest_peak(is_water));
}
