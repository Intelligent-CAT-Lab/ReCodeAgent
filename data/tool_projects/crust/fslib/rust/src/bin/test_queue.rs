use fslib::queue::Queue;
#[test]
fn test_queue_create() {
    let q: Queue<i32> = Queue::new();
    assert!(q.items.len()==0);
}
#[test]
fn test_queue_enqueue() {
    let mut q: Queue<i32> = Queue::new();
    let items = [0, 1, 2];
    for (i, &item) in items.iter().enumerate() {
        assert_eq!(q.len(), i);
        q.enqueue(item);
    }
    assert_eq!(q.dequeue(), Some(0));
    assert_eq!(q.len(), 2);
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.len(), 1);
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.len(), 0);
    assert_eq!(q.dequeue(), None);
    assert_eq!(q.dequeue(), None);
}
fn main() {
}