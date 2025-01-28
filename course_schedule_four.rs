pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {

    let n = num_courses as usize;

    let mut mat = vec![vec![false; n]; n];
    for p in prerequisites {
        let srce = p[0] as usize;
        let dest = p[1] as usize;
        mat[srce][dest] = true;
    }

    for mid in 0..n {
        for srce in 0..n {
            for dest in 0..n {
                mat[srce][dest] |= mat[srce][mid] & mat[mid][dest];
            }
        }
    }

    let mut ans: Vec<bool> = Vec::with_capacity(queries.len());
    for q in queries {
        let srce = q[0] as usize;
        let dest = q[1] as usize;
        ans.push(mat[srce][dest]);
    }
    
    return ans;
}

#[test]
fn tests() {
    let num_courses = 2;
    let prerequisites = vec![vec![1,0]];
    let queries = vec![vec![0,1],vec![1,0]];
    assert_eq!(vec![false, true], check_if_prerequisite(num_courses, prerequisites, queries));

    let num_courses = 2;
    let prerequisites = vec![];
    let queries = vec![vec![1,0],vec![0,1]];
    assert_eq!(vec![false, false], check_if_prerequisite(num_courses, prerequisites, queries));

    let num_courses = 3;
    let prerequisites = vec![vec![1,2],vec![1,0],vec![2,0]];
    let queries = vec![vec![1,0],vec![1,2]];
    assert_eq!(vec![true, true], check_if_prerequisite(num_courses, prerequisites, queries));
}
