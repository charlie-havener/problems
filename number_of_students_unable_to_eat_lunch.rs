use std::collections::VecDeque;

pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
    let mut students = VecDeque::from(students);
    let mut sandwich_idx = 0;
    loop {
        let count = students.len();
        if count == 0 {
            return 0;
        }

        for _ in 0..count {
            if *students.front().unwrap() == sandwiches[sandwich_idx] {
                students.pop_front();
                sandwich_idx += 1;
            } else {
                let t = students.pop_front().unwrap();
                students.push_back(t);
            }
        }
        
        // if all students were forced back into the queue.
        if students.len() == count {
            return students.len() as i32;
        }
    }
}

#[test]
fn tests() {
    let a = vec![1,1,0,0];
    let b = vec![0,1,0,1];
    assert_eq!(0, count_students(a,b));

    let a = vec![1,1,1,0,0,1];
    let b = vec![1,0,0,0,1,1];
    assert_eq!(3, count_students(a,b));
}

