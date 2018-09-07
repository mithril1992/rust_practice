use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::vec::Vec;
use std::collections::VecDeque;

type AsyncQueue<T> = Arc<Mutex<VecDeque<T>>>;

struct Task {
    id: u64,
    wait_time_in_millis: u64,
}

impl Task {
    fn new(id: u64) -> Task {
        Task {
            id: id,
            wait_time_in_millis: id * 450
        }
    }

    fn run(&self, thread_id: usize) {
        println!("begin task {} on thread {}", self.id, thread_id);
        thread::sleep(Duration::from_millis(self.wait_time_in_millis));
        // println!("end task {} on thread {}", self.id, thread_id);
    }
}

fn pop_from<T>(queue: &AsyncQueue<T>) -> Option<T> {
    let mut guard = queue.lock().unwrap();
    (*guard).pop_front()
}

fn main() {
    let tasks = (0..7).into_iter().map(|n| Task::new(n)).collect();
    let queue: AsyncQueue<Task> = Arc::new(Mutex::new(tasks));

    let handles: Vec<_> = (0..2).into_iter().map(|id| {
        let queue = queue.clone();
        thread::spawn(move ||{
            loop {
                match pop_from(&queue) {
                    Some(t) => t.run(id),
                    None => break,
                }
            }
        })   
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}