use std::collections::{hash_map::Entry, HashMap, VecDeque};

struct Logger {
    // HashSet entry is nightly only, so just use HashMap with empty vals
    set: HashMap<String, ()>,
    queue: VecDeque<(i32, String)>


}


impl Logger {
    fn new() -> Self {
        Self {
            set: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        
        println!("----");
        println!("{timestamp:?}, {message:?}");
        
        while let Some((t, m)) = self.queue.get(0) {
            if timestamp - t >= 10 {
                self.set.remove(m);
                self.queue.pop_front();
            } else {
                break;
            }
        }
        println!("{:?}, {:?}", self.set, self.queue);
        
        match self.set.entry(message.clone()) {
            Entry::Occupied(_) => return false,
            Entry::Vacant(v) => {
                self.queue.push_back((timestamp, message));
                v.insert(());
                return true;
            }
        }
    }
}
