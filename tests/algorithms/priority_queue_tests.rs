use broute::algorithms::PriorityQueue;

#[test]
fn test_priority_queue() {
    let mut queue = PriorityQueue::<usize, f64>::new();
    queue.push(1, 5.0);
    queue.push(2, 10.0);
    queue.push(3, 2.0);

    assert_eq!(queue.pop(), Some((3, 2.0)));
}

#[test]
fn test_priority_queue_with_strings() {
    let mut queue = PriorityQueue::<&str, usize>::new();
    queue.push("fish", 20);
    queue.push("eggs", 11);
    queue.push("bacon", 55);

    assert_eq!(queue.pop(), Some(("eggs", 11)));
}
