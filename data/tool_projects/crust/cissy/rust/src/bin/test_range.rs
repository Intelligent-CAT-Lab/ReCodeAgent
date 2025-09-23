use cissy::range::range::{RangeElement, RangeType};
use std::assert_eq;

#[test]
fn test_range() {
let mut list = None;
let mut buf = String::new();
let bufsize = 1024;

list = RangeElement::add_single(5, list);
assert!(!RangeElement::contains_num(4, &list));
assert!(RangeElement::contains_num(5, &list));

buf.clear();
RangeElement::list_to_string(&mut buf, bufsize, &list);
assert_eq!(buf, "[5]");

list = RangeElement::add_start_end(5, 10, list);
assert!(!RangeElement::contains_num(4, &list));
assert!(RangeElement::contains_num(5, &list));
assert!(RangeElement::contains_num(7, &list));
assert!(RangeElement::contains_num(10, &list));
assert!(!RangeElement::contains_num(12, &list));

buf.clear();
RangeElement::list_to_string(&mut buf, bufsize, &list);
assert_eq!(buf, "[5-10]");

list = RangeElement::add_start_end(5, 10, list);
list = RangeElement::add_single(15, list);
list = RangeElement::add_greater_equal(40, list);
assert!(!RangeElement::contains_num(1, &list));
assert!(RangeElement::contains_num(6, &list));
assert!(!RangeElement::contains_num(12, &list));
assert!(RangeElement::contains_num(50, &list));

buf.clear();
RangeElement::list_to_string(&mut buf, bufsize, &list);
assert_eq!(buf, "[5-10][15][40-]");

list = RangeElement::add_single(2, list);
list = RangeElement::add_start_end(2, 4, list);
buf.clear();
RangeElement::list_to_string(&mut buf, bufsize, &list);
assert_eq!(buf, "[2][2-4]");

let l2 = RangeElement::parse_int_ranges("2,2-4");
buf.clear();
RangeElement::list_to_string(&mut buf, bufsize, &l2);
assert_eq!(buf, "[2][2-4]");
}

fn main() {}