use std::collections::{HashMap, BTreeSet, hash_map::Entry};

struct NumberContainers {
    idx_store: HashMap<i32, i32>,
    finder: BTreeSet<(i32,i32)>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            idx_store: HashMap::new(),
            finder: BTreeSet::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        // if the index has a value currently, then that (value,index)
        // tuple needs to get removed from the BTrree
        // Then update the value at index and insert the new
        // tuple into the BTree
        let ent = self.idx_store.entry(index);
        if let Entry::Occupied(ref o) = ent {
            self.finder.remove(&(*o.get(), index));
        }
        ent.and_modify(|v| *v = number).or_insert(number);
        self.finder.insert((number, index));
    }

    fn find(&self, number: i32) -> i32 {
        // Get the first tuple in the BTree with the given number
        // If the range is empty, then return a -1
        self.finder.range(&(number,0)..=&(number,i32::MAX)).next().unwrap_or(&(-1,-1)).1
    }
}

