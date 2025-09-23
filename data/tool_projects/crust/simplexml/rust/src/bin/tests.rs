
use std::rc::Rc;
use std::cell::RefCell;


use SimpleXML::simple_vector::Vector;
use SimpleXML::simple_xml::{XMLElement, parse_xml_from_text};
#[test]
fn test_vector() {
    let a = 10;
    let b = 20;
    let c = 30;

    // Test allocate
    let mut v: Vector<&i32> = Vector::new(8);
    assert_eq!(v.size(), 0, "create size FAILED");
    assert!(v.data.capacity() >= 8, "create capacity FAILED");

    // Test push back
    v.push_back(&a);
    v.push_back(&b);
    assert_eq!(v.size(), 2, "insert element FAILED");
    assert_eq!(*v.get_element_at(0).unwrap(), &a, "insert element FAILED");
    assert_eq!(*v.get_element_at(1).unwrap(), &b, "insert element FAILED");

    // Test push front
    v.push_front(&a);
    v.push_front(&b);
    v.push_front(&c);
    assert_eq!(v.size(), 5);
    assert_eq!(*v.get_element_at(0).unwrap(), &c);
    assert_eq!(*v.get_element_at(1).unwrap(), &b);
    assert_eq!(*v.get_element_at(2).unwrap(), &a);
    assert_eq!(*v.get_element_at(3).unwrap(), &a);
    assert_eq!(*v.get_element_at(4).unwrap(), &b);

    // Test insert
    v.insert_at_index(&c, 2);
    assert_eq!(v.size(), 6);
    assert_eq!(*v.get_element_at(0).unwrap(), &c);
    assert_eq!(*v.get_element_at(1).unwrap(), &b);
    assert_eq!(*v.get_element_at(2).unwrap(), &c);
    assert_eq!(*v.get_element_at(3).unwrap(), &a);
    assert_eq!(*v.get_element_at(4).unwrap(), &a);
    assert_eq!(*v.get_element_at(5).unwrap(), &b);

    // Test remove
    v.remove_at_index(2);
    assert_eq!(v.size(), 5);
    assert_eq!(*v.get_element_at(0).unwrap(), &c);
    assert_eq!(*v.get_element_at(1).unwrap(), &b);
    assert_eq!(*v.get_element_at(2).unwrap(), &a);
    assert_eq!(*v.get_element_at(3).unwrap(), &a);
    assert_eq!(*v.get_element_at(4).unwrap(), &b);

    // Test re-allocate
    for _ in 0..100 {
        v.push_back(&a);
    }
    assert_eq!(v.size(), 105);
    assert_eq!(*v.get_element_at(0).unwrap(), &c);
    assert_eq!(*v.get_element_at(1).unwrap(), &b);
    assert_eq!(*v.get_element_at(2).unwrap(), &a);
    assert_eq!(*v.get_element_at(3).unwrap(), &a);
    assert_eq!(*v.get_element_at(4).unwrap(), &b);
    for i in 5..105 {
        assert_eq!(*v.get_element_at(i).unwrap(), &a);
    }

    // Test top
    assert_eq!(*v.top_back().unwrap(), &a);
    assert_eq!(*v.top_front().unwrap(), &c);
    assert_eq!(v.size(), 105);

    println!("PASSED Test vector1");
}

const SIZE: usize = 100000;
#[test]
fn test_vector2() {
    let mut v = Vector::new(8);
    
    // Initialize elements
    for i in 0..SIZE {
        let tag_name = format!("tag {}", i);
        let value = format!("value {}", i);
        let element = XMLElement::new(tag_name, value);
        v.push_back(element);
    }

    // Assert elements
    for i in 0..v.size() {
        let expected_tag = format!("tag {}", i);
        let expected_value = format!("value {}", i);
        
        let element = v.get_element_at(i);
        let element_ref = element.unwrap();

        assert_eq!(element_ref.tag_name, expected_tag);
        assert_eq!(element_ref.value, expected_value);
    }

    println!("PASSED Test vector2");
}
#[test]
fn test_xml() {
    let s = r#"<programmer>
    <name>Kien Nguyen Trung</name>
    <languages>
    <language>C</language>
    <language>C++</language>
    <language>Python</language>
    <language>Ruby</language>
    <language>Objective C</language>
    <language>Java</language>
    <language>Javascript</language>
    <language>Lua</language>
    <language>C#</language>
    <language>PHP</language>
    </languages>
    </programmer>"#;

    let elem = parse_xml_from_text(s).unwrap();
    let elem_ref = elem;
    
    assert_eq!(elem_ref.tag_name, "programmer");
    assert_eq!(elem_ref.children.size, 2);

    let child1 = elem_ref.children.get_element_at(0).unwrap();
    assert_eq!(child1.tag_name, "name");
    assert_eq!(child1.children.size, 0);
    assert_eq!(child1.value, "Kien Nguyen Trung");

    let child2 = elem_ref.children.get_element_at(1).unwrap();
    assert_eq!(child2.tag_name, "languages");
    assert_eq!(child2.children.size, 10);
    
    let list_languages = [
        "C", "C++", "Python", "Ruby", "Objective C", 
        "Java", "Javascript", "Lua", "C#", "PHP"
    ];

    for i in 0..10 {
        let child = child2.children.get_element_at(i).unwrap();
        assert_eq!(child.value, list_languages[i]);
        assert_eq!(child.tag_name, "language");
    }

    println!("PASSED Test parser xml");
}


fn main(){}