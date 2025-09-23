use libqueue::queue;

#[test]
fn crud_test() {
    println!("\n!!--CRUD Test--!!");
    let mut q = queue::Queue::new();
    println!("✅ Queue initialization");
    assert!(q.is_empty());
    println!("✅ Queue Empty");

    let a: Box<dyn std::any::Any> = Box::new(10);
    let b: Box<dyn std::any::Any> = Box::new(15);

    // Push `a` into the queue
    q.push(a);
    assert_eq!(q.size, 1);
    assert_eq!(
        q.front().unwrap().downcast_ref::<i32>(),
        q.back().unwrap().downcast_ref::<i32>()
    );
    println!("✅ Queue Push");

    // Push `b` into the queue
    q.push(b);
    assert_eq!(q.size, 2);
    assert_eq!(
        *q.front().unwrap().downcast_ref::<i32>().unwrap(),
        10
    );
    println!("✅ Queue Front");
    assert_eq!(
        *q.back().unwrap().downcast_ref::<i32>().unwrap(),
        15
    );
    println!("✅ Queue Back");

    // Pop the front of the queue
    let popped_value = q.pop().unwrap();
    assert_eq!(
        *popped_value.downcast_ref::<i32>().unwrap(),
        10
    );
    assert_eq!(q.size, 1);
    println!("✅ Queue Pop");

    // Free the queue
    q.free();
    println!("✅ Queue Free");
}

fn main() {
    
}
