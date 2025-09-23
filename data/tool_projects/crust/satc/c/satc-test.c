#include "assert.h"
#include "satc.h"

#define satc_nearest_hundredth(n) floor(n * 100 + 0.5) / 100

void satc_point_scale_xy_test () {
  satc_point_alloca(v);
  satc_point_set_xy(v, 5.0, 5.0);
  satc_point_scale_xy(v, 10.0, 10.0);
  assert(satc_point_get_x(v) == 50.0);
  assert(satc_point_get_y(v) == 50.0);
  satc_point_scale_xy(v, 0.0, 1.0);
  assert(satc_point_get_x(v) == 0.0);
  assert(satc_point_get_y(v) == 50.0);
  satc_point_scale_xy(v, 1.0, 0.0);
  assert(satc_point_get_x(v) == 0.0);
  assert(satc_point_get_y(v) == 0.0);
}

void satc_polygon_get_centroid_test () {
  {
    // Centroid of a square
    satc_point_array_alloca(points, 4);
    satc_point_alloca(pos);
    satc_point_set_xy(pos, 0.0, 0.0);
    satc_point_alloca(a);
    satc_point_set_xy(a, 0.0, 0.0);
    satc_point_alloca(b);
    satc_point_set_xy(b, 40.0, 0.0);
    satc_point_alloca(c);
    satc_point_set_xy(c, 40.0, 40.0);
    satc_point_alloca(d);
    satc_point_set_xy(d, 0.0, 40.0);
    points[0] = a;
    points[1] = b;
    points[2] = c;
    points[3] = d;
    satc_polygon_t *polygon = satc_polygon_create(pos, 4, points);
    double *centroid = satc_polygon_get_centroid(polygon);
    assert(satc_point_get_x(centroid) == 20.0);
    assert(satc_point_get_y(centroid) == 20.0);
    satc_point_destroy(centroid);
    satc_polygon_destroy(polygon);
  }

  {
    // Centroid of a triangle
    satc_point_array_alloca(points, 3);
    satc_point_alloca(pos);
    satc_point_set_xy(pos, 0.0, 0.0);
    satc_point_alloca(a);
    satc_point_set_xy(a, 0.0, 0.0);
    satc_point_alloca(b);
    satc_point_set_xy(b, 100.0, 0.0);
    satc_point_alloca(c);
    satc_point_set_xy(c, 50.0, 99.0);
    points[0] = a;
    points[1] = b;
    points[2] = c;
    satc_polygon_t *polygon = satc_polygon_create(pos, 3, points);
    double *centroid = satc_polygon_get_centroid(polygon);
    assert(satc_point_get_x(centroid) == 50.0);
    assert(satc_point_get_y(centroid) == 33.0);
    satc_point_destroy(centroid);
    satc_polygon_destroy(polygon);
  }
}

void satc_collision_test () {
  {
    // Circle-to-circle collision.
    satc_point_alloca_xy(c1_pos, 0.0, 0.0);
    satc_point_alloca_xy(c2_pos, 30.0, 0.0);
    satc_circle_t *circle1 = satc_circle_create(c1_pos, 20.0);
    satc_circle_t *circle2 = satc_circle_create(c2_pos, 20.0);
    satc_response_t *response = satc_response_create();
    bool collided = satc_test_circle_circle(circle1, circle2, response);
    assert(collided);
    assert(satc_point_get_x(response->overlap_v) == 10.0);
    assert(satc_point_get_y(response->overlap_v) == 0.0);
    satc_response_destroy(response);
    satc_circle_destroy(circle2);
    satc_circle_destroy(circle1);
  }

  {
    // Polygon-to-circle collision.
    satc_point_alloca_xy(c_pos, 50.0, 50.0);
    satc_point_alloca_xy(p_pos, 0.0, 0.0);
    satc_point_array_alloca(points, 4);
    satc_point_alloca_xy(a, 0.0, 0.0);
    satc_point_alloca_xy(b, 40.0, 0.0);
    satc_point_alloca_xy(c, 40.0, 40.0);
    satc_point_alloca_xy(d, 0.0, 40.0);
    points[0] = a;
    points[1] = b;
    points[2] = c;
    points[3] = d;
    satc_circle_t *circle = satc_circle_create(c_pos, 20.0);
    satc_polygon_t *polygon = satc_polygon_create(p_pos, 4, points);
    satc_response_t *response = satc_response_create();
    bool collided = satc_test_polygon_circle(polygon, circle, response);
    assert(collided);
    assert(satc_nearest_hundredth(response->overlap) == 5.86);
    assert(satc_nearest_hundredth(satc_point_get_x(response->overlap_v)) == 4.14);
    assert(satc_nearest_hundredth(satc_point_get_y(response->overlap_v)) == 4.14);
    satc_response_destroy(response);
    satc_circle_destroy(circle);
    satc_polygon_destroy(polygon);
  }

  {
    // Polygon-to-polygon collision.
    satc_point_alloca(circle_pos);
    satc_point_set_xy(circle_pos, 50.0, 50.0);
    satc_circle_t *circle = satc_circle_create(circle_pos, 20.0);
    satc_point_alloca(polygon_pos);
    satc_point_set_xy(polygon_pos, 0.0, 0.0);
    satc_point_array_alloca(polygon_points, 4);
    satc_point_alloca(polygon_point_1);
    satc_point_set_xy(polygon_point_1, 0.0, 0.0);
    satc_point_alloca(polygon_point_2);
    satc_point_set_xy(polygon_point_2, 40.0, 0.0);
    satc_point_alloca(polygon_point_3);
    satc_point_set_xy(polygon_point_3, 40.0, 40.0);
    satc_point_alloca(polygon_point_4);
    satc_point_set_xy(polygon_point_4, 0.0, 40.0);
    polygon_points[0] = polygon_point_1;
    polygon_points[1] = polygon_point_2;
    polygon_points[2] = polygon_point_3;
    polygon_points[3] = polygon_point_4;
    satc_polygon_t *polygon = satc_polygon_create(polygon_pos, 4, polygon_points);
    satc_response_t *response = satc_response_create();
    bool collided = satc_test_polygon_circle(polygon, circle, response);
    assert(collided);
    assert(satc_nearest_hundredth(response->overlap) == 5.86);
    assert(satc_nearest_hundredth(satc_point_get_x(response->overlap_v)) == 4.14);
    assert(satc_nearest_hundredth(satc_point_get_y(response->overlap_v)) == 4.14);
    satc_response_destroy(response);
    satc_polygon_destroy(polygon);
  }

  {
    // Polygon to polygon no collision.
    satc_point_alloca(pos_1);
    satc_point_set_xy(pos_1, 0.0, 0.0);
    satc_point_alloca(pos_2);
    satc_point_set_xy(pos_2, 100.0, 100.0);
    satc_box_t *box_1 = satc_box_create(pos_1, 20.0, 20.0);
    satc_polygon_t *polygon_1 = satc_box_to_polygon(box_1);
    satc_box_t *box_2 = satc_box_create(pos_2, 20.0, 20.0);
    satc_polygon_t *polygon_2 = satc_box_to_polygon(box_2);
    satc_response_t *response = satc_response_create();
    bool collided = satc_test_polygon_polygon(polygon_1, polygon_2, response);
    assert(!collided);
    satc_response_destroy(response);
    satc_polygon_destroy(polygon_2);
    satc_polygon_destroy(polygon_1);
    satc_box_destroy(box_2);
    satc_box_destroy(box_1);
  }
}

void satc_point_test () {
  {
    // Point in circle
    satc_point_alloca(circle_pos);
    satc_point_set_xy(circle_pos, 100.0, 100.0);
    satc_point_alloca(point_1);
    satc_point_set_xy(point_1, 0.0, 0.0);
    satc_point_alloca(point_2);
    satc_point_set_xy(point_2, 110.0, 110.0);
    satc_circle_t *circle = satc_circle_create(circle_pos, 20.0);
    bool test_1 = satc_point_in_circle(point_1, circle);
    assert(!test_1);
    bool test_2 = satc_point_in_circle(point_2, circle);
    assert(test_2);
    satc_circle_destroy(circle);
  }

  {
    // Point in polygon
    satc_point_alloca(triangle_pos);
    satc_point_set_xy(triangle_pos, 30.0, 0.0);
    satc_point_array_alloca(triangle_points, 3);
    satc_point_alloca(triangle_point_1);
    satc_point_set_xy(triangle_point_1, 0.0, 0.0);
    satc_point_alloca(triangle_point_2);
    satc_point_set_xy(triangle_point_2, 30.0, 0.0);
    satc_point_alloca(triangle_point_3);
    satc_point_set_xy(triangle_point_3, 0.0, 30.0);
    triangle_points[0] = triangle_point_1;
    triangle_points[1] = triangle_point_2;
    triangle_points[2] = triangle_point_3;
    satc_polygon_t *triangle = satc_polygon_create(triangle_pos, 3, triangle_points);
    satc_point_alloca(point_1);
    satc_point_set_xy(point_1, 0.0, 0.0);
    satc_point_alloca(point_2);
    satc_point_set_xy(point_2, 35.0, 5.0);
    bool test_1 = satc_point_in_polygon(point_1, triangle);
    assert(!test_1);
    bool test_2 = satc_point_in_polygon(point_2, triangle);
    assert(test_2);
    satc_polygon_destroy(triangle);
  }

  {
    // Point in polygon (small)

    satc_point_alloca(point);
    satc_point_set_xy(point, 1.0, 1.1);
    satc_point_alloca(polygon_pos);
    satc_point_set_xy(polygon_pos, 0.0, 0.0);
    satc_point_alloca(polygon_point_1);
    satc_point_set_xy(polygon_point_1, 2.0, 1.0);
    satc_point_alloca(polygon_point_2);
    satc_point_set_xy(polygon_point_2, 2.0, 2.0);
    satc_point_alloca(polygon_point_3);
    satc_point_set_xy(polygon_point_3, 1.0, 3.0);
    satc_point_alloca(polygon_point_4);
    satc_point_set_xy(polygon_point_4, 0.0, 2.0);
    satc_point_alloca(polygon_point_5);
    satc_point_set_xy(polygon_point_5, 0.0, 1.0);
    satc_point_alloca(polygon_point_6);
    satc_point_set_xy(polygon_point_6, 1.0, 0.0);
    satc_point_array_alloca(polygon_points, 6);
    polygon_points[0] = polygon_point_1;
    polygon_points[1] = polygon_point_2;
    polygon_points[2] = polygon_point_3;
    polygon_points[3] = polygon_point_4;
    polygon_points[4] = polygon_point_5;
    polygon_points[5] = polygon_point_6;
    satc_polygon_t *polygon = satc_polygon_create(polygon_pos, 6, polygon_points);
    bool test = satc_point_in_polygon(point, polygon);
    assert(test);
    satc_polygon_destroy(polygon);
  }
}

int main (int argc, char *argv[], char *envp[]) {
  satc_point_scale_xy_test();
  satc_polygon_get_centroid_test();
  satc_collision_test();
  satc_point_test();
  return EXIT_SUCCESS;
}
