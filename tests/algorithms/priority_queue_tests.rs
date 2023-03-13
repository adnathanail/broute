use broute::algorithms::PriorityQueue;

#[test]
fn test_priority_queue() {
    let mut queue = PriorityQueue::<usize>::new();
    queue.push(1, 5.0);
    queue.push(2, 10.0);
    queue.push(3, 2.0);

    assert_eq!(queue.pop(), Some((3, 2.0)));
}

#[test]
fn test_priority_queue_with_strings() {
    let mut queue = PriorityQueue::<&str>::new();
    queue.push("fish", 5.0);
    queue.push("eggs", 10.0);
    queue.push("bacon", 2.0);

    assert_eq!(queue.pop(), Some(("bacon", 2.0)));
}
