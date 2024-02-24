use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect::<Vec<_>>(),
            ranks: vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let root1 = self.find(x);
        let root2 = self.find(y);
        if root1 == root2 { return }
        match self.ranks[root1].cmp(&self.ranks[root2]) {
            Ordering::Less => self.parents[root1] = root2,
            Ordering::Greater => self.parents[root2] = root1,
            Ordering::Equal => {
                self.parents[root2] = root1;
                self.ranks[root1] += 1;
            }
        }
    }

    fn are_connected(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }

    fn deunion(&mut self, x: usize) {
        self.parents[x] = x;
        self.ranks[x] = 0;
    }
}

pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    meetings.sort_unstable_by(|a,b| a[2].cmp(&b[2]));

    let mut ptr1 = 0;
    let mut ptr2 = 0;

    let mut uf = UnionFind::new(n as usize);
    uf.union(0, first_person as usize);

    while ptr1 < meetings.len() {
        let current_time = meetings[ptr1][2];
        while  ptr2 < meetings.len() && current_time == meetings[ptr2][2] {
            ptr2 += 1;
        }

        // do the unioning
        for idx in ptr1..ptr2 {
            uf.union(meetings[idx][0] as usize, meetings[idx][1] as usize);
        }
        for idx in ptr1..ptr2 {
            if !uf.are_connected(meetings[idx][0] as usize, 0) {
                uf.deunion(meetings[idx][0] as usize);
                uf.deunion(meetings[idx][1] as usize);
            }
        }

        ptr1 = ptr2;
    }
    
    let ans = (0..n).filter(|v| uf.are_connected(*v as usize, 0)).collect::<Vec<_>>();    
    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let meetings = vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]];
        let first_person = 1;
        assert_eq!(vec![0,1,2,3,5], find_all_people(n, meetings, first_person));
        let n = 4;
        let meetings = vec![vec![3,1,3],vec![1,2,2],vec![0,3,3]];
        let first_person = 3;
        assert_eq!(vec![0,1,3], find_all_people(n, meetings, first_person));
        let n = 5;
        let meetings = vec![vec![3,4,2],vec![1,2,1],vec![2,3,1]];
        let first_person = 1;
        assert_eq!(vec![0,1,2,3,4], find_all_people(n, meetings, first_person));
        let n = 4;
        let meetings = vec![vec![0,2,3],vec![1,2,2],vec![0,3,3]];
        let first_person = 3;
        assert_eq!(vec![0,2,3], find_all_people(n, meetings, first_person));
        let n = 6;
        let meetings = vec![vec![0,2,1],vec![1,3,1],vec![4,5,1]];
        let first_person = 1;
        assert_eq!(vec![0,1,2,3], find_all_people(n, meetings, first_person));
    }
}
