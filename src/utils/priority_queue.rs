use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, PartialEq)]
struct PriorityQueueItem<V: PartialEq, P: PartialEq + PartialOrd> {
    value: V,
    priority: P,
}

impl<V: PartialEq, P: PartialEq + PartialOrd> Eq for PriorityQueueItem<V, P> {}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl<V: PartialEq, P: PartialEq + PartialOrd> PartialOrd<Self> for PriorityQueueItem<V, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl<V: PartialEq, P: PartialEq + PartialOrd> Ord for PriorityQueueItem<V, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

/// Struct representing a min-heap priority queue
#[derive(Debug)]
pub struct PriorityQueue<V: PartialEq, P: PartialEq + PartialOrd> {
    heap: BinaryHeap<PriorityQueueItem<V, P>>,
}

impl<V: PartialEq, P: PartialEq + PartialOrd> PriorityQueue<V, P> {
    /// Create a new priority queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    /// Add an item to the queue with the given priority
    pub fn push(&mut self, value: V, priority: P) {
        self.heap.push(PriorityQueueItem { value, priority })
    }
    /// Remove and return the next item from the queue
    pub fn pop(&mut self) -> Option<(V, P)> {
        self.heap.pop().map(|s| (s.value, s.priority))
    }
}

impl<V: PartialEq, P: PartialEq + PartialOrd> Default for PriorityQueue<V, P> {
    fn default() -> Self {
        Self::new()
    }
}
