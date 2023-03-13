use broute::algorithms::PriorityQueue;

#[test]
fn test_priority_queue() {
    let mut queue = PriorityQueue::new();
    queue.push(1, 5.0);
    queue.push(2, 10.0);
    queue.push(3, 2.0);

    assert_eq!(queue.pop(), Some((3, 2.0)));
}
