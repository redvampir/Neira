use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct ScheduledTask {
    priority: u8,
    id: String,
    input: String,
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct TaskScheduler {
    heap: BinaryHeap<ScheduledTask>,
}

impl TaskScheduler {
    pub fn enqueue(&mut self, id: String, input: String, priority: u8) {
        self.heap.push(ScheduledTask { priority, id, input });
    }

    pub fn next(&mut self) -> Option<(String, String)> {
        self.heap.pop().map(|t| (t.id, t.input))
    }
}
