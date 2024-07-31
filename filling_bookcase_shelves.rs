pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {

    let mut total = vec![0; books.len() + 1];
    total[1] = books[0][1];
    
    for b in 2..=books.len() {
        let thick = books[b-1][0];
        let height = books[b-1][1];

        let mut max_height = height;
        let mut rem = shelf_width - thick;
        
        total[b] = total[b-1] + max_height;
        let mut p = b-1;
        while p > 0 && rem - books[p-1][0] >= 0 {
            max_height = max_height.max(books[p-1][1]);
            total[b] = total[b].min(max_height + total[p-1]);
            p -= 1;
            rem -= books[p][0];
        }
    }
    return *total.last().unwrap();
}

#[test]
fn tests() {
    let b = vec![vec![1,1],vec![2,3],vec![2,3],vec![1,1],vec![1,1],vec![1,1],vec![1,2]];
    let w = 4;
    assert_eq!(6, min_height_shelves(b,w));

    let b = vec![vec![1,3],vec![2,4],vec![3,2]];
    let w = 6;
    assert_eq!(4, min_height_shelves(b,w));
}

