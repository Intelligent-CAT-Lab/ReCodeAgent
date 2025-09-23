use hydra::hydra::*;

fn test_start(name: &str) {
    println!("-- Testing {}", name);
}

#[test]
fn test_new_command() {
    test_start("NewCommand");
    let cmd = Command::new('k', "name".to_string(), "command".to_string());
    assert_eq!(cmd.key, 'k');
    assert_eq!(cmd.name, "name");
    assert_eq!(cmd.command, "command");
    assert!(cmd.children.is_none());
    assert!(cmd.next.is_none());
}

#[test]
fn test_command_add_child() {
    test_start("CommandAddChild");
    let mut parent = Command::new('\0', "".to_string(), "".to_string());
    let first_child = Command::new('a', "".to_string(), "".to_string());
    let second_child = Command::new('b', "".to_string(), "".to_string());
    let third_child = Command::new('c', "".to_string(), "".to_string());

    command_add_child(&mut parent, first_child);
    command_add_child(&mut parent, second_child);
    command_add_child(&mut parent, third_child);

    assert!(parent.children.is_some());
    let first = parent.children.as_ref().unwrap();
    assert_eq!(first.key, 'a');
    assert!(first.next.is_some());
    let second = first.next.as_ref().unwrap();
    assert_eq!(second.key, 'b');
    assert!(second.next.is_some());
    let third = second.next.as_ref().unwrap();
    assert_eq!(third.key, 'c');
    assert!(third.next.is_none());
}

fn main() {}