use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PriorityQueueItem {
    priority: usize,
    value: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.value.cmp(&other.value))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO genericise
#[derive(Debug)]
pub struct PriorityQueue {
    heap: BinaryHeap<PriorityQueueItem>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, cost: usize, position: usize) {
        self.heap.push(PriorityQueueItem {
            priority: cost,
            value: position,
        })
    }

    pub fn force_pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|val| val.value)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
