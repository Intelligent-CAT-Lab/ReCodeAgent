use satc::satc;
use std::f64;
fn satc_nearest_hundredth(n: f64) -> f64 {
(n * 100.0).floor() / 100.0
}
#[test]
fn satc_point_scale_xy_test() {
let mut v = [5.0, 5.0];
v = satc::satc_point_scale_xy(&v, 10.0, 10.0);
assert_eq!(v[0], 50.0);
assert_eq!(v[1], 50.0);
v = satc::satc_point_scale_xy(&v, 0.0, 1.0);
assert_eq!(v[0], 0.0);
assert_eq!(v[1], 50.0);
v = satc::satc_point_scale_xy(&v, 1.0, 0.0);
assert_eq!(v[0], 0.0);
assert_eq!(v[1], 0.0);
}
#[test]
fn satc_polygon_get_centroid_test() {
{
let pos = [0.0, 0.0];
let points = vec![[0.0, 0.0], [40.0, 0.0], [40.0, 40.0], [0.0, 40.0]];
let polygon = satc::satc_polygon_create(pos, points);
let centroid = satc::satc_polygon_get_centroid(&polygon);
assert_eq!(centroid[0], 20.0);
assert_eq!(centroid[1], 20.0);
}
{
let pos = [0.0, 0.0];
let points = vec![[0.0, 0.0], [100.0, 0.0], [50.0, 99.0]];
let polygon = satc::satc_polygon_create(pos, points);
let centroid = satc::satc_polygon_get_centroid(&polygon);
assert_eq!(centroid[0], 50.0);
assert_eq!(centroid[1], 33.0);
}
}
#[test]
fn satc_collision_test() {
{
let c1_pos = [0.0, 0.0];
let c2_pos = [30.0, 0.0];
let circle1 = satc::satc_circle_create(c1_pos, 20.0);
let circle2 = satc::satc_circle_create(c2_pos, 20.0);
let mut response = satc::satc_response_create();
let collided = satc::satc_test_circle_circle(&circle1, &circle2, &mut response);
assert!(collided);
assert_eq!(response.overlap_v[0], 10.0);
assert_eq!(response.overlap_v[1], 0.0);
}
{
let c_pos = [50.0, 50.0];
let p_pos = [0.0, 0.0];
let points = vec![[0.0, 0.0], [40.0, 0.0], [40.0, 40.0], [0.0, 40.0]];
let circle = satc::satc_circle_create(c_pos, 20.0);
let polygon = satc::satc_polygon_create(p_pos, points);
let mut response = satc::satc_response_create();
let collided = satc::satc_test_polygon_circle(&polygon, &circle, &mut response);
assert!(collided);
assert_eq!(satc_nearest_hundredth(response.overlap), 5.86);
assert_eq!(satc_nearest_hundredth(response.overlap_v[0]), 4.14);
assert_eq!(satc_nearest_hundredth(response.overlap_v[1]), 4.14);
}
{
let circle_pos = [50.0, 50.0];
let circle = satc::satc_circle_create(circle_pos, 20.0);
let polygon_pos = [0.0, 0.0];
let polygon_points = vec![
[0.0, 0.0],
[40.0, 0.0],
[40.0, 40.0],
[0.0, 40.0],
];
let polygon = satc::satc_polygon_create(polygon_pos, polygon_points);
let mut response = satc::satc_response_create();
let collided = satc::satc_test_polygon_circle(&polygon, &circle, &mut response);
assert!(collided);
assert_eq!(satc_nearest_hundredth(response.overlap), 5.86);
assert_eq!(satc_nearest_hundredth(response.overlap_v[0]), 4.14);
assert_eq!(satc_nearest_hundredth(response.overlap_v[1]), 4.14);
}
{
let pos_1 = [0.0, 0.0];
let pos_2 = [100.0, 100.0];
let box_1 = satc::satc_box_create(pos_1, 20.0, 20.0);
let polygon_1 = satc::satc_box_to_polygon(&box_1);
let box_2 = satc::satc_box_create(pos_2, 20.0, 20.0);
let polygon_2 = satc::satc_box_to_polygon(&box_2);
let mut response = satc::satc_response_create();
let collided = satc::satc_test_polygon_polygon(&polygon_1, &polygon_2, &mut response);
assert!(!collided);
}
}
#[test]
fn satc_point_test() {
{
let circle_pos = [100.0, 100.0];
let point_1 = [0.0, 0.0];
let point_2 = [110.0, 110.0];
let circle = satc::satc_circle_create(circle_pos, 20.0);
let test_1 = satc::satc_point_in_circle(&point_1, &circle);
assert!(!test_1);
let test_2 = satc::satc_point_in_circle(&point_2, &circle);
assert!(test_2);
}
{
let triangle_pos = [30.0, 0.0];
let triangle_points = vec![[0.0, 0.0], [30.0, 0.0], [0.0, 30.0]];
let triangle = satc::satc_polygon_create(triangle_pos, triangle_points);
let point_1 = [0.0, 0.0];
let point_2 = [35.0, 5.0];
let test_1 = satc::satc_point_in_polygon(&point_1, &triangle);
assert!(!test_1);
let test_2 = satc::satc_point_in_polygon(&point_2, &triangle);
assert!(test_2);
}
{
let point = [1.0, 1.1];
let polygon_pos = [0.0, 0.0];
let polygon_points = vec![
[2.0, 1.0],
[2.0, 2.0],
[1.0, 3.0],
[0.0, 2.0],
[0.0, 1.0],
[1.0, 0.0],
];
let polygon = satc::satc_polygon_create(polygon_pos, polygon_points);
let test = satc::satc_point_in_polygon(&point, &polygon);
assert!(test);
}
}
fn main() {}