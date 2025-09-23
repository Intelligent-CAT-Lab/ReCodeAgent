use geofence::geofence::is_position_in_geofence;
use geofence::geofence::Point;

#[derive(Debug)]
struct TestCase {
    point: Point,
    vertices: Vec<Point>,
    inside: bool,
}
fn check_inside(test_case: &TestCase) {
    assert!(is_position_in_geofence(&test_case.point, &test_case.vertices) == test_case.inside);
}
#[test]
fn test() {
    let poly1 = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 1.0, y: -1.0 },
        Point { x: -1.0, y: -1.0 },
        Point { x: -1.0, y: 1.0 },
    ];
    
    let poly2 = vec![
        Point { x: 0.0, y: 1.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: -1.0 },
        Point { x: -1.0, y: 0.0 },
    ];
    
    let poly_testcases = vec![
        TestCase { point: Point { x: 0.0, y: 0.0 }, vertices: poly1.clone(), inside: true },
        TestCase { point: Point { x: -1.0, y: 0.0 }, vertices: poly1.clone(), inside: true },
        TestCase { point: Point { x: 0.0, y: 0.5 }, vertices: poly2.clone(), inside: true },
        TestCase { point: Point { x: 5.0, y: 0.5 }, vertices: poly2.clone(), inside: false },
    ];
    
    println!("====== Polygon Tests ======");
    for test_case in poly_testcases {
        check_inside(&test_case);
    }
}
fn main(){}