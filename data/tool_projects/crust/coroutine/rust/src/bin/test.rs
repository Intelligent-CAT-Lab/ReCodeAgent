use std::any::Any;

use coroutine::coroutine::{
    Schedule,
    coroutine_open,
    coroutine_close,
    coroutine_new,
    coroutine_resume,
    coroutine_status,
    coroutine_running,
    coroutine_yield,
    COROUTINE_DEAD,
};

struct Args {
    n: i32,
}

fn foo(schedule: &mut Schedule, data: &mut dyn Any) {
    let args = data.downcast_mut::<Args>().expect("Expected `Args` type");

    let current_id = coroutine_running(schedule);
    assert!(
        current_id >= 0,
        "Expected a valid running coroutine ID (>=0), got {}",
        current_id
    );

    let start = args.n;

    for i in 0..5 {
        println!("coroutine {} : {}", current_id, start + i);
        
        coroutine_yield(schedule);
        
        let resumed_id = coroutine_running(schedule);
        assert_eq!(
            resumed_id, current_id,
            "After yielding and resuming, expected the same coroutine ID"
        );
    }
}

fn test(schedule: &mut Schedule) {
    let arg1 = Args { n: 0 };
    let arg2 = Args { n: 100 };

    let co1 = coroutine_new(schedule, foo, Box::new(arg1));
    let co2 = coroutine_new(schedule, foo, Box::new(arg2));

    assert!(
        co1 >= 0,
        "Expected a nonnegative coroutine ID for co1, got {}",
        co1
    );
    assert!(
        co2 >= 0,
        "Expected a nonnegative coroutine ID for co2, got {}",
        co2
    );
    assert_ne!(
        co1, co2,
        "co1 and co2 should be different coroutine IDs"
    );

    println!("main start");

    while coroutine_status(schedule, co1) != COROUTINE_DEAD
        && coroutine_status(schedule, co2) != COROUTINE_DEAD
    {
        coroutine_resume(schedule, co1);
        coroutine_resume(schedule, co2);
    }

    assert_eq!(
        coroutine_status(schedule, co1),
        COROUTINE_DEAD,
        "co1 should be dead after loop ends"
    );
    assert_eq!(
        coroutine_status(schedule, co2),
        COROUTINE_DEAD,
        "co2 should be dead after loop ends"
    );

    println!("main end");
}


#[test]
fn test_main() {
    let mut schedule = coroutine_open();
    test(&mut schedule);
    coroutine_close(schedule);
}
fn main(){}
