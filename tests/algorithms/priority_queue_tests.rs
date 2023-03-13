use broute::algorithms::PriorityQueue;

#[test]
fn test_priority_queue() {
    let mut queue = PriorityQueue::new();
    queue.push(5.0, 1);
    queue.push(10.0, 2);
    queue.push(2.0, 3);

    assert_eq!(queue.pop(), Some((3, 2.0)));
}
