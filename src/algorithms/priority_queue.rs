use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, PartialEq)]
struct PriorityQueueItem<V: PartialEq> {
    value: V,
    priority: f64,
}

impl<V: PartialEq> Eq for PriorityQueueItem<V> {}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl<V: PartialEq> PartialOrd<Self> for PriorityQueueItem<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl<V: PartialEq> Ord for PriorityQueueItem<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Equal)
    }
}

#[derive(Debug)]
pub struct PriorityQueue<V: PartialEq> {
    heap: BinaryHeap<PriorityQueueItem<V>>,
}

impl<V: PartialEq> PriorityQueue<V> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn push(&mut self, value: V, priority: f64) {
        self.heap.push(PriorityQueueItem { value, priority })
    }
    pub fn pop(&mut self) -> Option<(V, f64)> {
        self.heap.pop().map(|s| (s.value, s.priority))
    }
}

impl<V: PartialEq> Default for PriorityQueue<V> {
    fn default() -> Self {
        Self::new()
    }
}
