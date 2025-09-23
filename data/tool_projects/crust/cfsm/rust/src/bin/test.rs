use cfsm::cfsm;
#[derive(Default)]
struct StateOperationCounter {
    pub enter_calls: i32,
    pub leave_calls: i32,
    pub event_calls: i32,
    pub process_calls: i32,
    pub last_event_id: i32,
}

static mut state_a: StateOperationCounter = StateOperationCounter{    
    enter_calls: 0,
    leave_calls: 0,
    event_calls: 0,
    process_calls: 0,
    last_event_id: 0,};

static mut state_b: StateOperationCounter = StateOperationCounter{    
    enter_calls: 0,
    leave_calls: 0,
    event_calls: 0,
    process_calls: 0,
    last_event_id: 0,};

fn test_cfsm_init_is_safe_to_use<T>(fsm_instance: &mut cfsm::CfsmCtx<T>) {
    fsm_instance.cfsm_init(None);
    fsm_instance.cfsm_process();
    fsm_instance.cfsm_event(0x12345678);
}

fn test_cfsm_init_should_clear_handler(fsm_instance: &mut cfsm::CfsmCtx<u8>) {
    let dummy_instance_data = Box::new(42);
    fsm_instance.cfsm_init(Some(dummy_instance_data));
    assert!(fsm_instance.on_event.is_none());
    assert!(fsm_instance.on_process.is_none());
    assert!(fsm_instance.on_leave.is_none());
    assert!(fsm_instance.ctx_ptr.is_some());
}


fn test_cfsm_transition_should_set_enter_handler_only<T>(fsm_instance: &mut cfsm::CfsmCtx<T>) {
    fsm_instance.cfsm_transition(Some(state_only_on_enter));
    assert!(fsm_instance.on_event.is_none());
    assert!(fsm_instance.on_process.is_none());
    assert!(fsm_instance.on_leave.is_none());
}


fn test_cfs_transition_a_b_a<T>(fsm_instance: &mut cfsm::CfsmCtx<T>) {
    fsm_instance.cfsm_transition(Some(state_a_on_enter));
    assert!(fsm_instance.on_event == Some(state_a_on_event));
    assert!(fsm_instance.on_process == Some(state_a_on_process));
    assert!(fsm_instance.on_leave == Some(state_a_on_leave));
    unsafe {
        assert_eq!(state_a.enter_calls, 1);
        assert_eq!(state_a.leave_calls, 0);
        assert_eq!(state_a.process_calls, 0);
        assert_eq!(state_a.event_calls, 0);
        assert_eq!(state_b.enter_calls, 0);
        assert_eq!(state_b.leave_calls, 0);
        assert_eq!(state_b.process_calls, 0);
        assert_eq!(state_b.event_calls, 0);
    }

    fsm_instance.cfsm_transition(Some(state_b_on_enter));
    assert!(fsm_instance.on_event == Some(state_b_on_event));
    assert!(fsm_instance.on_process == Some(state_b_on_process));
    assert!(fsm_instance.on_leave == Some(state_b_on_leave));
    unsafe {
        assert_eq!(state_a.enter_calls, 1);
        assert_eq!(state_a.leave_calls, 1);
        assert_eq!(state_a.process_calls, 0);
        assert_eq!(state_a.event_calls, 0);
        assert_eq!(state_b.enter_calls, 1);
        assert_eq!(state_b.leave_calls, 0);
        assert_eq!(state_b.process_calls, 0);
        assert_eq!(state_b.event_calls, 0);
    }

    fsm_instance.cfsm_transition(Some(state_a_on_enter));
    assert!(fsm_instance.on_event == Some(state_a_on_event));
    assert!(fsm_instance.on_process == Some(state_a_on_process));
    assert!(fsm_instance.on_leave == Some(state_a_on_leave));
    unsafe {
        assert_eq!(state_a.enter_calls, 2);
        assert_eq!(state_a.leave_calls, 1);
        assert_eq!(state_a.process_calls, 0);
        assert_eq!(state_a.event_calls, 0);
        assert_eq!(state_b.enter_calls, 1);
        assert_eq!(state_b.leave_calls, 1);
        assert_eq!(state_b.process_calls, 0);
        assert_eq!(state_b.event_calls, 0);
    }
}
#[test]
fn test_main() {
    let mut fsm_instance: cfsm::CfsmCtx<u8> = cfsm::CfsmCtx::new();
    test_cfsm_init_is_safe_to_use(&mut fsm_instance);
    test_cfsm_init_should_clear_handler(&mut fsm_instance);
    test_cfsm_transition_should_set_enter_handler_only(&mut fsm_instance);
    test_cfs_transition_a_b_a(&mut fsm_instance);
}

fn state_only_on_enter<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    // do nothing
}

fn state_a_on_enter<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    let fsm_instance = fsm.unwrap();
    fsm_instance.on_event = Some(state_a_on_event);
    fsm_instance.on_process = Some(state_a_on_process);
    fsm_instance.on_leave = Some(state_a_on_leave);
    unsafe {
        state_a.enter_calls = state_a.event_calls + 1;
    }
}

fn state_a_on_event<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>, event_id: i32) {
    unsafe {
        state_a.event_calls = state_a.event_calls + 1;
    }
}

fn state_a_on_process<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    unsafe {
        state_a.process_calls = state_a.event_calls + 1;
    }
}

fn state_a_on_leave<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    unsafe {
        state_a.leave_calls = state_a.event_calls + 1;
    }
}

fn state_b_on_enter<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    let fsm_instance = fsm.unwrap();
    fsm_instance.on_event = Some(state_a_on_event);
    fsm_instance.on_process = Some(state_a_on_process);
    fsm_instance.on_leave = Some(state_a_on_leave);
    unsafe {
        state_b.enter_calls = state_b.event_calls + 1;
    }
}

fn state_b_on_event<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>, event_id: i32) {
    unsafe {
        state_b.event_calls = state_b.event_calls + 1;
    }
}

fn state_b_on_process<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    unsafe {
        state_b.process_calls = state_b.event_calls + 1;
    }
}

fn state_b_on_leave<T>(fsm: Option<Box<&mut cfsm::CfsmCtx<T>>>) {
    unsafe {
        state_b.leave_calls = state_b.event_calls + 1;
    }
}

fn main(){}