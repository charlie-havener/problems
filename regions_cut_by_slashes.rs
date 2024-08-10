pub fn regions_by_slashes(grid: Vec<String>) -> i32 {

    let mut converted: Vec<Vec<usize>> = Vec::with_capacity(grid.len());

    for (r, s) in grid.iter().enumerate() {
        converted.push(Vec::new());
        let mut s = s.chars();
        while let Some(c) = s.next() {
            match c {
                '/' => converted[r].push(1),
                '\\' => {
                    converted[r].push(1);
                },
                ' ' => converted[r].push(0),
                _ => unreachable!(),
            }
        }
    }
    println!("{converted:?}");


    return 0;
}


#[test]
fn test() {
    //let grid = vec![String::from(" /"),String::from("/ ")];
    //assert_eq!(2, regions_by_slashes(grid));
    //let grid = vec![String::from(" /"),String::from("  ")];
    //assert_eq!(1, regions_by_slashes(grid));
    let grid = vec![String::from("/\\"),String::from("\\/")];
    assert_eq!(5, regions_by_slashes(grid));
}
