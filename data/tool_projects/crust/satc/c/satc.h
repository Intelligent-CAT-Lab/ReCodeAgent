/* vim: set ft=c: */

/** @file */

#ifndef __SATC__
#define __SATC__

#include "stdio.h"
#include "stdlib.h"
#include "stdbool.h"
#include "math.h"
#include "float.h"

// -------------------------------------
// Forward declarations for the structs.
// -------------------------------------

/** The type of a circle struct. */
typedef struct satc_circle satc_circle_t;
/** The type of a polygon struct. */
typedef struct satc_polygon satc_polygon_t;
/** The type of a box struct. */
typedef struct satc_box satc_box_t;
/** The type of a response struct. */
typedef struct satc_response satc_response_t;

/** A circle shape, with a position and radius. */
struct satc_circle {
  /** The shape type of the struct. */
  int type;
  /** The position of the circle. */
  double *pos;
  /** The radius of the circle. */
  double r;
};

/** A polygon shape, with a position, angle, offset, and points. */
struct satc_polygon {
  /** The shape type of the struct. */
  int type;
  /** The position of the polygon. */
  double *pos;
  /** The number of points in the polygon. */
  size_t num_points;
  /**
   * The list of points in the polygon, as an array of arrays of doubles.
   *
   * Use `satc_polygon_copy_points` to change this. Or, if you need to do it
   * manually, call `_satc_polygon_recalc` afterward.
   */
  double **points;
  /**
   * The angle of rotation of the polygon.
   *
   * Use `satc_polygon_set_angle` to change this. Or, if you need to do it
   * manually, call `_satc_polygon_recalc` afterward.
   */
  double angle;
  /**
   * The offset as an array of doubles (a point) of the polygon.
   *
   * Use `satc_polygon_set_offset` to change this. Or, if you need to do it
   * manually, call `_satc_polygon_recalc` afterward.
   */
  double *offset;
  /**
   * The number of calculated points for the polygon. This will match the
   * number of points.
   *
   * This should not be modified manually.
   */
  size_t num_calc_points;
  /**
   * An array of arrays of doubles (array of points) representing the
   * calculated points of the polygon. This will match the number of points.
   *
   * This should not be modified manually.
   */
  double **calc_points;
  /**
   * The number of calculated edges for the polygon. This will match the
   * number of points.
   *
   * This should not be modified manually.
   */
  size_t num_edges;
  /**
   * An array of arrays of doubles (array of points) representing the
   * calculated edges of the polygon. This will match the number of points.
   *
   * This should not be modified manually.
   */
  double **edges;
  /**
   * The number of calculated normals for the polygon. This will match the
   * number of points.
   *
   * This should not be modified manually.
   */
  size_t num_normals;
  /**
   * An array of arrays of doubles (array of points) representing the
   * calculated normals of the polygon. This will match the number of points.
   *
   * This should not be modified manually.
   */
  double **normals;
};

/** A box shape, with a position, width, and height. */
struct satc_box {
  /** The shape type of the struct. */
  int type;
  /** The position of the box. */
  double *pos;
  /** The width of the box. */
  double w;
  /** The height of the box. */
  double h;
};

/** A response, representing an overlap between two shapes. */
struct satc_response {
  /** The first shape participating in the collision. */
  void *a;
  /** The second shape participating in the collision. */
  void *b;
  /** The length of overlap of the collision. */
  double overlap;
  /** The unit vector of the overlap of the collision. */
  double *overlap_n;
  /**
   * The unit vector of the overlap of the collision, scaled by the length of
   * overlap.
   */
  double *overlap_v;
  /** True if shape `a` is entirely within shape `b`. */
  bool a_in_b;
  /** True if shape `b` is entirely within shape `a`. */
  bool b_in_a;
};

// ------
// Macros
// ------

/** Denotes an undefined type in a struct with a `type` field. */
#define satc_type_none 0
/** Denotes a circle type in a struct with a `type` field. */
#define satc_type_circle 1
/** Denotes a polygon type in a struct with a `type` field. */
#define satc_type_polygon 2
/** Denotes a box type in a struct with a `type` field. */
#define satc_type_box 3

/** Given a struct, returns an int representing the type.. */
#define satc_shape_get_type(s) s->type;

/**
 * Creates an array of doubles, with undefined values. Since it uses `alloca`,
 * the array will automatically be deallocated when it falls out of scope.
 *
 * This macro is considered a statement.
 *
 * @param name the variable name for the array.
 * @param size the number of doubles in the array.
 */
#define satc_double_array_alloca(name, size) \
  double *name = NULL; \
  name = (double *) alloca(sizeof(double) * size);

/**
 * Creates an array of pointers to `double *` (a point), with undefined,
 * not-yet-allocated pointer values.
 *
 * Since it uses `alloca`, the array will automatically be deallocated when it
 * falls out of scope.
 *
 * The members will not be automatically deallocated when the array falls out
 * of scope, unless the members were allocated with `alloca` too!
 *
 * This macro is considered a statement.
 *
 * @param name the variable name for the array.
 * @param size the number of pointers to points in the array.
 */
#define satc_point_array_alloca(name, size) \
  double **name = NULL; \
  name = (double **) alloca(sizeof(double *) * size);

/**
 * Creates a pointer to an array of doubles (a point), with undefined `x` and
 * `y` values.
 *
 * Since it uses `alloca`, the point will automatically be deallocated when it
 * falls out of scope.
 *
 * This macro is considered a statement.
 *
 * @param name the variable name for the point.
 */
#define satc_point_alloca(name) \
  double *name = NULL; \
  name = (double *) alloca(sizeof(double) * 2);

/**
 * Creates a pointer to an array of doubles (a point), with defined `x` and `y`
 * values.
 *
 * Since it uses `alloca`, the point will automatically be deallocated when it
 * falls out of scope.
 *
 * This macro is considered a statement.
 *
 * @param name the variable name for the array.
 * @param x the x value, as a double, for the point.
 * @param y the y value, as a double, for the point.
 */
#define satc_point_alloca_xy(name, x, y) \
  satc_point_alloca(name); \
  satc_point_set_xy(name, x, y);

/** Denotes a left voronoi region, for polygon collision detection. */
#define SATC_LEFT_VORONOI_REGION -1
/** Denotes a middle voronoi region, for polygon collision detection. */
#define SATC_MIDDLE_VORONOI_REGION 0
/** Denotes a right voronoi region, for polygon collision detection. */
#define SATC_RIGHT_VORONOI_REGION 1

/** The index of the double array which contains the `x` value.. */
#define SATC_POINT_X 0
/** The index of the double array which contains the `y` value.. */
#define SATC_POINT_Y 1

/**
 * Get the `x` value from an array of doubles (a point).
 *
 * @param p the array of doubles (point) to fetch from.
 * @return the `x` value as a double.
 */
#define satc_point_get_x(p) p[SATC_POINT_X]
/**
 * Set the `x` value in an array of doubles (a point).
 *
 * This macro is considered a statement.
 *
 * @param p the array of doubles (point) whose value should change.
 * @param v the `x` value to change.
 */
#define satc_point_set_x(p, v) p[SATC_POINT_X] = v;
/**
 * Get the `y` value from an array of doubles (a point).
 *
 * @param p the array of doubles (point) to fetch from.
 * @return the `y` value as a double.
 */
#define satc_point_get_y(p) p[SATC_POINT_Y]
/**
 * Set the `y` value in an array of doubles (a point).
 *
 * This macro is considered a statement.
 *
 * @param p the array of doubles (point) whose value should change.
 * @param v the `y` value to change.
 */
#define satc_point_set_y(p, v) p[SATC_POINT_Y] = v;
/**
 * Set both the `x` and `y` values in an array of doubles (a point).
 *
 * This macro is considered a statement.
 *
 * @param p the array of doubles (point) whose value should change.
 * @param x the `x` value to change.
 * @param y the `y` value to change.
 */
#define satc_point_set_xy(p, x, y) \
  p[SATC_POINT_X] = x; \
  p[SATC_POINT_Y] = y;
/**
 * Given a point, allocate a separate array of doubles (a point) with
 * equivalent values.
 *
 * This macro is considered a statement.
 *
 * You are responsible for deallocating the point.
 *
 * @param p the point to clone.
 */
#define satc_point_clone(p) satc_point_create(satc_point_get_x(p), satc_point_get_y(p))

// --------------------------------------------------------------------
// Forward declarations of certain functions called by other functions.
// --------------------------------------------------------------------

/** Forward declaration of `satc_polygon_set_points`. */
satc_polygon_t *satc_polygon_set_points (satc_polygon_t *polygon, size_t num_points, double **points);
/** Forward declaration of `_satc_polygon_recalc`. */
satc_polygon_t *_satc_polygon_recalc (satc_polygon_t *polygon);
/** Forward declaration of `satc_box_create`. */
satc_box_t *satc_box_create (double *pos, double w, double h);
/** Forward declaration of `satc_box_destroy`. */
void satc_box_destroy (satc_box_t *box);
/** Forward declaration of `satc_box_to_polygon`. */
satc_polygon_t *satc_box_to_polygon (satc_box_t *box);
/** Forward declaration of `satc_test_polygon_polygon`. */
bool satc_test_polygon_polygon (satc_polygon_t *a, satc_polygon_t *b, satc_response_t *response);

// ---------
// Functions
// ---------

/**
 * Copy the values of some array of doubles (a point) `q`, onto some point `p`.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param q the point whose values to use
 * @return the mutated point.
 */
double *satc_point_copy (double *p, double *q) {
  satc_point_set_xy(p, satc_point_get_x(q), satc_point_get_y(q));
  return p;
}

/**
 * Modify the values of some array of doubles (a point) `p` so they match a
 * point perpendicular to the original point.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @return the mutated point.
 */
double *satc_point_perp (double *p) {
  double x = satc_point_get_y(p);
  double y = -satc_point_get_x(p);
  satc_point_set_xy(p, x, y);
  return p;
}

/**
 * Modify the values of some array of doubles (a point) `p` so they match an
 * inverse point to the original point.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @return the mutated point.
 */
double *satc_point_reverse (double *p) {
  satc_point_set_xy(p, -satc_point_get_x(p), -satc_point_get_y(p));
  return p;
}

/**
 * Find the dot product of two arrays of doubles (points).
 *
 * @param p the first point.
 * @param q the second point.
 * @return the dot product, as a double.
 */
#define satc_point_dot(p, q) satc_point_get_x(p) * satc_point_get_x(q) + satc_point_get_y(p) * satc_point_get_y(q)
/**
 * Find the length, squared, of an array of doubles (a point).
 *
 * @param p the point to query.
 * @return the length squared, as a double.
 */
#define satc_point_len2(p) satc_point_dot(p, p)
/**
 * Find the length, of an array of doubles (a point). This is slightly more
 * intensive than finding the length squared.
 *
 * @param p the point to query.
 * @return the length, as a double.
 */
#define satc_point_len(p) sqrt(satc_point_len2(p))

/**
 * Find the sum of two arrays of doubles (points), and replace the values of the
 * first point to match the sum.
 *
 * `p` will be mutated.
 *
 * @param p the first point.
 * @param q the second point.
 * @return the mutated point.
 */
double *satc_point_add (double *p, double *q) {
  satc_point_set_xy(p, satc_point_get_x(p) + satc_point_get_x(q), satc_point_get_y(p) + satc_point_get_y(q));
  return p;
}

/**
 * Find the difference of two arrays of doubles (points), and replace the
 * values of the first point to match the difference.
 *
 * `p` will be mutated.
 *
 * @param p the first point.
 * @param q the second point.
 * @return the mutated point.
 */
double *satc_point_sub (double *p, double *q) {
  satc_point_set_xy(p, satc_point_get_x(p) - satc_point_get_x(q), satc_point_get_y(p) - satc_point_get_y(q));
  return p;
}

/**
 * Scale some array of doubles (a point) by some value, `x`, and some value,
 * `y`, and replace the values of the point to match the scaled result.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param x the `x` value to scale by.
 * @param y the `y` value to scale by.
 * @return the mutated point.
 */
double *satc_point_scale_xy (double *p, double x, double y) {
  satc_point_set_xy(p, satc_point_get_x(p) * x, satc_point_get_y(p) * y);
  return p;
}

/**
 * Scale some array of doubles (a point) by some value, `x`, along both axes.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param x the `x` value to scale by, horizontally, and vertically.
 * @return the mutated point.
 */
double *satc_point_scale_x (double *p, double x) {
  return satc_point_scale_xy(p, x, x);
}

/**
 * Allocates a new array of doubles (a point). The values will be undefined.
 *
 * You are responsible for deallocating the point.
 *
 * @return the newly-created point.
 */
double *satc_point_create (double x, double y) {
  double *point = NULL;
  point = (double *) malloc(sizeof(double) * 2);
  satc_point_set_xy(point, x, y);
  return point;
}

/**
 * Deallocates an array of doubles (a point).
 *
 * @param point the point to deallocate.
 */
void satc_point_destroy (double *point) {
  free(point);
}

/**
 * Rotates a point by some angle.
 *
 * `p` will be mutated.
 *
 * @param p the point to rotate.
 * @param angle the angle to rotate by.
 */
double *satc_point_rotate (double *p, double angle) {
  double x = satc_point_get_x(p);
  double y = satc_point_get_y(p);
  satc_point_set_xy(p, x * cos(angle) - y * sin(angle), x * sin(angle) - y * cos(angle));
  return p;
}

/**
 * Normalize some array of doubles (a point) into a unit vector.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @return the normalized point.
 */
double *satc_point_normalize (double *p) {
  double d = satc_point_len(p);
  if (d > 0) satc_point_set_xy(p, satc_point_get_x(p) / d, satc_point_get_y(p) / d);
  return p;
}

/**
 * Project some array of doubles (a point) `q` onto `p`.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param q the point used to project.
 * @return the projected point point.
 */
double *satc_point_project (double *p, double *q) {
  double amt = satc_point_dot(p, q) / satc_point_len2(q);
  satc_point_set_xy(p, amt * satc_point_get_x(p), amt * satc_point_get_y(p));
  return p;
}

/**
 * Project some array of doubles (a point) `q` onto `p`, but the result is not
 * divided by the length of `q` squared.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param q the point used to project.
 * @return the projected point point.
 */
double *satc_point_project_n (double *p, double *q) {
  double amt = satc_point_dot(p, q);
  satc_point_set_xy(p, amt * satc_point_get_x(p), amt * satc_point_get_y(p));
  return p;
}

/**
 * Reflect some array of doubles (a point) along some axis.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param axis the angle to reflect on.
 * @return the normalized point.
 */
double *satc_point_reflect (double *p, double *axis) {
  double x = satc_point_get_x(p);
  double y = satc_point_get_y(p);
  satc_point_project(p, axis);
  satc_point_scale_x(p, 2.0);
  satc_point_set_xy(p, satc_point_get_x(p) - x, satc_point_get_y(p) - y);
  return p;
}

/**
 * Reflect some array of doubles (a point) along some axis. When projecting, the
 * value is not divided by the length of `p` squared.
 *
 * `p` will be mutated.
 *
 * @param p the point to mutate.
 * @param axis the angle to reflect on.
 * @return the normalized point.
 */
double *satc_point_reflect_n (double *p, double *axis) {
  double x = satc_point_get_x(p);
  double y = satc_point_get_y(p);
  satc_point_project_n(p, axis);
  satc_point_scale_x(p, 2.0);
  satc_point_set_xy(p, satc_point_get_x(p) - x, satc_point_get_y(p) - y);
  return p;
}

/**
 * Create a struct representing a circle, with a given position and radius.
 *
 * You are responsible for deallocating the circle.
 *
 * @param pos the position of the circle.
 * @param r the radius of the circle.
 * @return a circle struct.
 */
satc_circle_t *satc_circle_create (double *pos, double r) {
  satc_circle_t *circle = NULL;
  circle = (satc_circle_t *) malloc(sizeof(satc_circle_t));
  circle->type = satc_type_circle;
  circle->pos = satc_point_clone(pos);
  circle->r = r;
  return circle;
}

/**
 * Deallocates a struct representing a circle.
 *
 * @param circle the circle to deallocate.
 */
void satc_circle_destroy (satc_circle_t *circle) {
  satc_point_destroy(circle->pos);
  circle->type = satc_type_none;
  circle->pos = NULL;
  circle->r = -1.0;
  free(circle);
}

/**
 * Returns a struct representing a rectangular polygon which is equivalent to
 * the bounding box of a circle.
 *
 * You are responsible for deallocating the polygon.
 *
 * @param circle the circle whose bounding box is of interest.
 * @return a polygon struct.
 */
satc_polygon_t *satc_circle_get_aabb (satc_circle_t *circle) {
  double r = circle->r;
  satc_point_alloca(corner);
  satc_point_copy(corner, circle->pos);
  satc_point_set_xy(corner, satc_point_get_x(corner) - r, satc_point_get_y(corner) - r);
  satc_box_t *box = satc_box_create(corner, r * 2.0, r * 2.0);
  satc_polygon_t *polygon = satc_box_to_polygon(box);
  satc_box_destroy(box);
  return polygon;
}

/**
 * Create a struct representing a polygon, with a given position and points.
 *
 * The points passed in are copied, and so you must handle the deallocation of
 * the passed-in points.
 *
 * You are responsible for deallocating the polygon.
 *
 * @param pos the position of the polygon.
 * @param num_points the number of points provided.
 * @param points an array of arrays of doubles (array of points).
 * @return a polygon struct.
 */
satc_polygon_t *satc_polygon_create (double *pos, size_t num_points, double **points) {
  satc_polygon_t *polygon = NULL;
  polygon = (satc_polygon_t *) malloc(sizeof(satc_polygon_t));
  polygon->type = satc_type_polygon;
  polygon->pos = satc_point_clone(pos);
  polygon->angle = 0.0;
  polygon->offset = satc_point_create(0.0, 0.0);

  polygon->num_points = 0;
  polygon->points = NULL;
  polygon->num_calc_points = 0;
  polygon->calc_points = NULL;
  polygon->num_edges = 0;
  polygon->edges = NULL;
  polygon->num_normals = 0;
  polygon->normals = NULL;

  satc_polygon_set_points(polygon, num_points, points);

  return polygon;
}

/**
 * Deallocates a struct representing a polygon.
 *
 * @param polygon the polygon to deallocate.
 */
void satc_polygon_destroy (satc_polygon_t *polygon) {
  satc_point_destroy(polygon->pos);
  satc_point_destroy(polygon->offset);

  size_t i = 0;
  for (; i < polygon->num_points; i++) {
    satc_point_destroy(polygon->points[i]);
  }

  free(polygon->points);

  polygon->type = satc_type_none;
  polygon->pos = NULL;
  polygon->num_points = -1;
  polygon->points = NULL;
  polygon->angle = 0.0;
  polygon->offset = NULL;
  free(polygon);
}

/**
 * Replace a polygon struct's list of points.
 *
 * The old points will be deallocated. The new points will be deallocated when
 * the polygon is deallocated.
 *
 * The calculated values of the polygon will be recalculated.
 *
 * The points passed in are copied, and so you must handle the deallocation of
 * the passed-in points.
 *
 * For internal use.
 *
 * @param polygon the polygon whose points to replace.
 * @param num_points the number of new points.
 * @param points an array of arrays of doubles (array of points).
 */
void _satc_polygon_copy_points (satc_polygon_t *polygon, size_t num_points, double **points) {
  if (polygon->points != NULL) {
    size_t i = 0;
    for (; i < polygon->num_points; i++) satc_point_destroy(polygon->points[i]);
    free(polygon->points);
    polygon->num_points = 0;
    polygon->points = NULL;
  }

  double **new_points = NULL;
  new_points = (double **) malloc(sizeof(double *) * num_points);
  size_t i = 0;
  for (; i < num_points; i++) new_points[i] = satc_point_clone(points[i]);
  polygon->num_points = num_points;
  polygon->points = new_points;
  _satc_polygon_recalc(polygon);
}

/**
 * Clear out the calculated points of a polygon and replace them with some
 * number of preallocated empty points.
 *
 * The old calculated points will be deallocated. The new points will be
 * deallocated when the polygon is deallocated.
 *
 * For internal use.
 *
 * @param polygon the polygon whose calculated points should be replaced.
 * @param num_calc_points the number of pristine points to add.
 */
void _satc_polygon_reset_calc_points (satc_polygon_t *polygon, size_t num_calc_points) {
  if (polygon->calc_points != NULL) {
    size_t i = 0;
    for (; i < polygon->num_calc_points; i++) satc_point_destroy(polygon->calc_points[i]);
    free(polygon->calc_points);
    polygon->num_calc_points = 0;
    polygon->calc_points = NULL;
  }

  double **new_calc_points = NULL;
  new_calc_points = (double **) malloc(sizeof(double *) * num_calc_points);

  size_t i = 0;
  for (; i < num_calc_points; i++) {
    new_calc_points[i] = satc_point_create(0.0, 0.0);
  }

  polygon->num_calc_points = num_calc_points;
  polygon->calc_points = new_calc_points;
}

/**
 * Replace the calculated edges of a polygon with some number of pristine
 * points representing edges.
 *
 * The old calculated edges will be deallocated. The new edges will be
 * deallocated when the polygon is deallocated.
 *
 * For internal use.
 *
 * @param polygon the polygon whose edges should be replaced.
 * @param num_edges the number of pristine points to add.
 */
void _satc_polygon_reset_edges (satc_polygon_t *polygon, size_t num_edges) {
  if (polygon->edges != NULL) {
    size_t i = 0;
    for (; i < polygon->num_edges; i++) satc_point_destroy(polygon->edges[i]);
    free(polygon->edges);
    polygon->num_edges = 0;
    polygon->edges = NULL;
  }

  double **new_edges = NULL;
  new_edges = (double **) malloc(sizeof(double *) * num_edges);

  size_t i = 0;
  for (; i < num_edges; i++) {
    new_edges[i] = satc_point_create(0.0, 0.0);
  }

  polygon->num_edges = num_edges;
  polygon->edges = new_edges;
}

/**
 * Replace the calculated normals of a polygon.
 *
 * The old calculated normals will be deallocated. The new calculated normals
 * will be deallocated when the polygon is deallocated.
 *
 * For internal use.
 *
 * @param polygon the polygon whose calculated normals should be replaced.
 * @param num_normals the number of pristine normals to add.
 */
void _satc_polygon_reset_normals (satc_polygon_t *polygon, size_t num_normals) {
  if (polygon->normals != NULL) {
    size_t i = 0;
    for (; i < polygon->num_normals; i++) satc_point_destroy(polygon->normals[i]);
    free(polygon->normals);
    polygon->num_normals = 0;
    polygon->normals = NULL;
  }

  double **new_normals = NULL;
  new_normals = (double **) malloc(sizeof(double *) * num_normals);

  size_t i = 0;
  for (; i < num_normals; i++) {
    new_normals[i] = satc_point_create(0.0, 0.0);
  }

  polygon->num_normals = num_normals;
  polygon->normals = new_normals;
}

/**
 * Given a polygon and number of points, it will clear out all of the
 * calculated values, replace the points already on the polygon, and
 * recalculate all the calculated values.
 *
 * All deallocation and allocation is handled for you.
 *
 * The points passed in are copied, and so you must handle the deallocation of
 * the passed-in points.
 *
 * @param polygon the polygon whose points should be replaced.
 * @param num_points the number of new points.
 * @param points an array of arrays of doubles (array of points).
 * @return the polygon passed in.
 */
satc_polygon_t *satc_polygon_set_points (satc_polygon_t *polygon, size_t num_points, double **points) {
  bool length_changed = num_points != polygon->num_points;
  if (length_changed) {
    _satc_polygon_reset_calc_points(polygon, num_points);
    _satc_polygon_reset_edges(polygon, num_points);
    _satc_polygon_reset_normals(polygon, num_points);
  }

  _satc_polygon_copy_points(polygon, num_points, points);
  return polygon;
}

/**
 * Set the angle of rotation of the polygon. All calculated values will be
 * recalculated.
 *
 * @param polygon the polygon to rotate.
 * @param angle the angle to rotate by.
 * @return the passed-in polygon.
 */
satc_polygon_t *satc_polygon_set_angle (satc_polygon_t *polygon, double angle) {
  polygon->angle = angle;
  _satc_polygon_recalc(polygon);
  return polygon;
}

/**
 * Set the offset of the polygon. All calculated values will be recalculated.
 *
 * @param polygon the polygon to offset.
 * @param offset the array of doubles (a point) to offset by.
 * @return the passed-in polygon.
 */
satc_polygon_t *satc_polygon_set_offset (satc_polygon_t *polygon, double *offset) {
  satc_point_copy(polygon->offset, offset);
  _satc_polygon_recalc(polygon);
  return polygon;
}

/**
 * In lieu of setting the angle of the polygon, you can actually rotate all of
 * the points. All calculated values will be recalculated.
 *
 * @param polygon the polygon to offset.
 * @param angle the array of doubles (a point) to offset by.
 * @return the passed-in polygon.
 */
satc_polygon_t *satc_polygon_rotate (satc_polygon_t *polygon, double angle) {
  size_t i = 0;
  for (; i < polygon->num_points; i++) satc_point_rotate(polygon->points[i], angle);
  _satc_polygon_recalc(polygon);
  return polygon;
}

/**
 * In lieu of setting the offset of the polygon, you can actually translate all
 * of the points by some `x` and `y`. All calculated values will be
 * recalculated.
 *
 * @param polygon the polygon to offset.
 * @param x the horizontal amount to translate by.
 * @param y the vertical amount to translate by.
 * @return the passed-in polygon.
 */
satc_polygon_t *satc_polygon_translate (satc_polygon_t *polygon, double x, double y) {
  size_t i = 0;
  for (; i < polygon->num_points; i++) satc_point_set_xy(polygon->points[i], x, y);
  _satc_polygon_recalc(polygon);
  return polygon;
}

/**
 * Recalculates all the calculated values for a struct representing a polygon
 * shape.
 *
 * For internal use.
 *
 * @param polygon a polygon whose values should be recalculated.
 * @return the passed-in polygon.
 */
satc_polygon_t *_satc_polygon_recalc (satc_polygon_t *polygon) {
  double **calc_points = polygon->calc_points;
  double **edges = polygon->edges;
  double **normals = polygon->normals;
  double **points = polygon->points;
  double *offset = polygon->offset;
  double angle = polygon->angle;
  size_t num_points = polygon->num_points;
  size_t i = 0;
  for (; i < num_points; i++) {
    double *calc_point = satc_point_copy(calc_points[i], points[i]);
    satc_point_add(calc_point, offset);
    if (angle != 0.0) satc_point_rotate(calc_point, angle);
  }

  i = 0;
  for (; i < num_points; i++) {
    double *p1 = calc_points[i];
    double *p2 = (i < num_points - 1) ? calc_points[i + 1] : calc_points[0];
    double *edge = edges[i];
    satc_point_copy(edge, p2);
    satc_point_sub(edge, p1);
    double *normal = normals[i];
    satc_point_copy(normal, edge);
    satc_point_perp(normal);
    satc_point_normalize(normal);
  }

  return polygon;
}

/**
 * Returns a polygon struct representing the bounding box of a polygon.
 *
 * You are responsible for deallocating the returned polygon.
 *
 * @param polygon the polygon whose bounding box should be calculated.
 * @return a polygon representing the bounding box.
 */
satc_polygon_t *satc_polygon_get_aabb (satc_polygon_t *polygon) {
  double x_min = satc_point_get_x(polygon->points[0]);
  double y_min = satc_point_get_y(polygon->points[0]);
  double x_max = x_min;
  double y_max = x_max;
  size_t i = 1;
  for (; i < polygon->num_points; i++) {
    double x = satc_point_get_x(polygon->points[i]);
    double y = satc_point_get_y(polygon->points[i]);
    if (x < x_min) x_min = x;
    if (x > x_max) x_max = x;
    if (y < y_min) y_min = y;
    if (y > y_max) y_max = y;
  }

  satc_point_alloca(pos);
  satc_point_set_xy(pos, satc_point_get_x(pos) + x_min, satc_point_get_y(pos) + y_min);
  satc_box_t *box = satc_box_create(pos, x_max - x_min, y_max - y_min);
  satc_polygon_t *new_polygon = satc_box_to_polygon(box);
  satc_box_destroy(box);
  return new_polygon;
}

/**
 * Get the "centroid" (the estimated center) of a polygon as an array of
 * doubles (a point).
 *
 * You are responsible for deallocating the point.
 *
 * @param polygon the polygon whose centroid should be calculated.
 * @return an array of doubles (a point) representing the centroid.
 */
double *satc_polygon_get_centroid (satc_polygon_t *polygon) {
  double **points = polygon->calc_points;
  size_t len = polygon->num_calc_points;
  double cx = 0.0;
  double cy = 0.0;
  double ar = 0.0;
  size_t i = 0;
  for (; i < len; i++) {
    double *p1 = points[i];
    double *p2 = (i == len - 1) ? points[0] : points[i + 1];
    double a = satc_point_get_x(p1) * satc_point_get_y(p2) - satc_point_get_x(p2) * satc_point_get_y(p1);
    cx += (satc_point_get_x(p1) + satc_point_get_x(p2)) * a;
    cy += (satc_point_get_y(p1) + satc_point_get_y(p2)) * a;
    ar += a;
  }

  ar = ar * 3.0;
  cx = cx / ar;
  cy = cy / ar;

  return satc_point_create(cx, cy);
}

/**
 * Creates a struct representing a box shape.
 *
 * @param pos the position of the box, from the top left.
 * @param w the width of the box.
 * @param h the height of the box.
 * @return a box struct.
 */
satc_box_t *satc_box_create (double *pos, double w, double h) {
  satc_box_t *box = NULL;
  box = (satc_box_t *) malloc(sizeof(satc_box_t));
  box->type = satc_type_box;
  box->pos = satc_point_clone(pos);
  box->w = w;
  box->h = h;
  return box;
}

/**
 * Deallocates a struct representing a box shape.
 *
 * @param box the box to deallocate.
 */
void satc_box_destroy (satc_box_t *box) {
  satc_point_destroy(box->pos);
  box->type = satc_type_none;
  box->pos = NULL;
  box->w = -1.0;
  box->h = -1.0;
  free(box);
}

/**
 * Returns a polygon representing the box.
 *
 * You are responsible for deallocating the polygon.
 *
 * @param box the box to generate a polygon of.
 * @return a polygon representing the box.
 */
satc_polygon_t *satc_box_to_polygon (satc_box_t *box) {
  double *pos = box->pos;
  double w = box->w;
  double h = box->h;
  double **points = NULL;
  points = (double **) alloca(sizeof(double *) * 4);
  satc_point_alloca(nw);
  satc_point_alloca(ne);
  satc_point_alloca(se);
  satc_point_alloca(sw);
  satc_point_set_xy(nw, 0.0, 0.0);
  satc_point_set_xy(ne, w, 0.0);
  satc_point_set_xy(se, w, h);
  satc_point_set_xy(sw, 0.0, h);
  points[0] = nw;
  points[1] = ne;
  points[2] = se;
  points[3] = sw;
  return satc_polygon_create(pos, 4, points);
}

/**
 * Creates a struct representing a collision response.
 *
 * Instead of setting the values manually, you should use a `satc_test_*`
 * function, which will set the values for you.
 *
 * @return a struct representing a collision response.
 */
satc_response_t *satc_response_create () {
  satc_response_t *response = NULL;
  response = (satc_response_t *) malloc(sizeof(satc_response_t));
  response->a = NULL;
  response->b = NULL;
  response->overlap_n = satc_point_create(0.0, 0.0);
  response->overlap_v = satc_point_create(0.0, 0.0);
  response->overlap = DBL_MAX;
  response->a_in_b = true;
  response->b_in_a = true;
  return response;
}

/**
 * Deallocates a struct representing a collision response.
 *
 * `response->a` and `response->b` are not deallocated for you!
 *
 * @param response the response to deallocate.
 */
void satc_response_destroy (satc_response_t *response) {
  satc_point_destroy(response->overlap_n);
  satc_point_destroy(response->overlap_v);

  response->a = NULL;
  response->b = NULL;
  response->overlap_n = NULL;
  response->overlap_v = NULL;
  response->overlap = DBL_MAX;
  response->a_in_b = true;
  response->b_in_a = true;
  free(response);
}

/**
 * Project the array of arrays of doubles (array of points) onto an axis.
 *
 * @param len the number of points.
 * @param points the array of arrays of doubles (points) to project.
 * @param normal the array of doubles (a point) of the normal.
 * @param result the arary of doubles (a point) representing the result.
 */
void satc_flatten_points_on (size_t len, double **points, double *normal, double *result) {
  double min = DBL_MAX;
  double max = -DBL_MAX;

  size_t i = 0;
  for (; i < len; i++) {
    double dot = satc_point_dot(points[i], normal);
    if (dot < min) min = dot;
    if (dot > max) max = dot;
  }

  result[0] = min;
  result[1] = max;
}

/**
 * Figure out if two arrays of arrays of doubles (two arrays of points)
 * represent a separating axis.
 *
 * Primarily meant for internal use.
 *
 * @param a_pos an array of doubles (a point).
 * @param b_pos an array of doubles (a point).
 * @param a_len the number of points in `a_points`.
 * @param a_points an array of arrays of doubles (an array of points).
 * @param b_len the number of points in `b_points`.
 * @param b_points an array of arrays of doubles (an array of points).
 * @param axis the axis as an array of doubles (a point).
 * @param response the collision response to mutate.
 * @return whether this represents a separating axis, as a boolean.
 */
bool satc_is_separating_axis (double *a_pos, double *b_pos, size_t a_len, double **a_points, size_t b_len, double **b_points, double *axis, satc_response_t *response) {
  // Allocate temporary variables.
  satc_double_array_alloca(range_a, 2);
  satc_double_array_alloca(range_b, 2);
  satc_point_alloca(offset_v);

  // The magnitude of the offset between the two polygons.
  satc_point_copy(offset_v, b_pos);
  satc_point_sub(offset_v, a_pos);
  double projected_offset = satc_point_dot(offset_v, axis);
  // Project the polygons onto the axis.
  satc_flatten_points_on(a_len, a_points, axis, range_a);
  satc_flatten_points_on(b_len, b_points, axis, range_b);
  // Move B's range to its position relative to A.
  range_b[0] += projected_offset;
  range_b[1] += projected_offset;
  // Check if there is a gap. If there is, this is a separating axis and we can stop.
  if (range_a[0] > range_b[1] || range_b[0] > range_a[1]) {
    return true;
  }

  // This is not a separating axis. If we're calculating a response, calculate the overlap.
  if (response != NULL) {
    double overlap = 0;
    // A starts further left than B.
    if (range_a[0] < range_b[0]) {
      response->a_in_b = false;
      // A ends before B does. We have to pull A out of B.
      if (range_a[1] < range_b[1]) {
        overlap = range_a[1] - range_b[1];
        response->b_in_a = false;
      // B is fully inside A. Pick the shortest way out.
      } else {
        double option_1 = range_a[1] - range_b[0];
        double option_2 = range_b[1] - range_a[0];
        overlap = option_1 < option_2 ? option_1 : -option_2;
      }
    // B starts further left than A.
    } else {
      response->b_in_a = false;
      // B ends before A ends. We have to push A out of B.
      if (range_a[1] > range_b[1]) {
        overlap = range_a[0] - range_b[1];
        response->a_in_b = false;
      // A is fully inside B.  Pick the shortest way out.
      } else {
        double option_1 = range_a[1] - range_b[0];
        double option_2 = range_b[1] - range_a[0];
        overlap = option_1 < option_2 ? option_1 : -option_2;
      }
    }

    // If this is the smallest amount of overlap we've seen so far, set it as the minimum overlap.
    double abs_overlap = fabs(overlap);
    if (abs_overlap < response->overlap) {
      response->overlap = abs_overlap;
      satc_point_copy(response->overlap_n, axis);
      if (overlap < 0) {
        satc_point_reverse(response->overlap_n);
      }
    }
  }

  return false;
}

/**
 * Figure out if an array of doubles (a point) representing a line, and another
 * point represent either a left, middle, or right voronoi region.
 *
 * @param line an array of doubles (a point) representing a line.
 * @param point an array of doubles (a point).
 * @return an integer (-1, 0, or 1) representing which voronoi region was found.
 */
int satc_voronoi_region (double *line, double *point) {
  double len_2 = satc_point_len2(line);
  double dp = satc_point_dot(point, line);
  // If the point is beyond the start of the line, it is in the left voronoi region.
  if (dp < 0) return SATC_LEFT_VORONOI_REGION;
  // If the point is beyond the end of the line, it is in the right voronoi region.
  if (dp > len_2) return SATC_RIGHT_VORONOI_REGION;
  return SATC_MIDDLE_VORONOI_REGION;
}

/**
 * Returns true if an array of doubles (a point) is inside of a circle.
 *
 * @param p an array of doubles (a point).
 * @param c a circle.
 * @return true if point is inside of circle, false otherwise.
 */
bool satc_point_in_circle (double *point, satc_circle_t *circle) {
  satc_point_alloca(difference_v);
  satc_point_copy(difference_v, point);
  satc_point_sub(difference_v, circle->pos);
  double radius_sq = circle->r * circle->r;
  double distance_sq = satc_point_len2(difference_v);
  return distance_sq <= radius_sq;
}

/**
 * Creates a very small polygon representing a point, for collision purposes.
 *
 * For internal use.
 *
 * You are responsible for deallocating the polygon.
 *
 * @return a polygon.
 */
satc_polygon_t *_satc_test_point_create () {
  satc_point_alloca(zero);
  satc_point_set_xy(zero, 0.0, 0.0);
  satc_box_t *box = satc_box_create(zero, 0.000001, 0.000001);
  satc_polygon_t *polygon = satc_box_to_polygon(box);
  satc_box_destroy(box);
  return polygon;
}

/**
 * Returns true if an array of doubles (a point) is inside of a polygon.
 *
 * @param p an array of doubles (a point).
 * @param polygon a polygon.
 * @return true if point is inside of polygon, false otherwise.
 */
bool satc_point_in_polygon (double *point, satc_polygon_t *polygon) {
  satc_polygon_t *test_point = _satc_test_point_create();
  satc_point_copy(test_point->pos, point);
  satc_response_t *response = satc_response_create();

  bool result = satc_test_polygon_polygon(test_point, polygon, response);
  if (result) result = response->a_in_b;

  satc_response_destroy(response);
  satc_polygon_destroy(test_point);

  return result;
}

/**
 * Returns true if a circle overlaps with another circle.
 *
 * @param a a circle.
 * @param b another circle.
 * @param response a response representing the collision.
 * @return true if the circles overlap, false otherwise.
 */
bool satc_test_circle_circle (satc_circle_t *a, satc_circle_t *b, satc_response_t *response) {
  satc_point_alloca(difference_v);
  satc_point_copy(difference_v, b->pos);
  satc_point_sub(difference_v, a->pos);
  double total_radius = a->r + b->r;
  double total_radius_sq = total_radius * total_radius;
  double distance_sq = satc_point_len2(difference_v);
  if (distance_sq > total_radius_sq) return false;
  if (response != NULL) {
    double distance = sqrt(distance_sq);
    response->a = a;
    response->b = b;
    response->overlap = total_radius - distance;
    satc_point_normalize(difference_v);
    satc_point_copy(response->overlap_n, difference_v);
    satc_point_scale_x(difference_v, response->overlap);
    satc_point_copy(response->overlap_v, difference_v);
    response->a_in_b = (a->r <= b->r) && (distance <= b->r - a->r);
    response->b_in_a = (b->r <= a->r) && (distance <= a->r - b->r);
  }

  return true;
}

/**
 * Checks to see if one polygon and one circle are overlapping.
 *
 * @param polygon a polygon.
 * @param circle a circle.
 * @param response the response object to set with collision data.
 */
bool satc_test_polygon_circle (satc_polygon_t *polygon, satc_circle_t *circle, satc_response_t *response) {
  satc_point_alloca(circle_pos);
  satc_point_copy(circle_pos, circle->pos);
  satc_point_sub(circle_pos, polygon->pos);
  double radius = circle->r;
  double radius2 = radius * radius;
  double **points = polygon->calc_points;
  size_t len = polygon->num_calc_points;
  satc_point_alloca(edge);
  satc_point_alloca(point);

  size_t i = 0;
  for (; i < len; i++) {
    size_t next = (i == len - 1) ? 0 : i + 1;
    size_t prev = (i == 0) ? len - 1 : i - 1;
    double overlap = 0.0;
    double *overlap_n = NULL;

    satc_point_copy(edge, polygon->edges[i]);
    satc_point_copy(point, circle_pos);
    satc_point_sub(point, points[i]);

    if (response != NULL && satc_point_len2(point) > radius2) {
      response->a_in_b = false;
    }

    int region = satc_voronoi_region(edge, point);
    if (region == SATC_LEFT_VORONOI_REGION) {
      satc_point_copy(edge, polygon->edges[prev]);
      satc_point_alloca(point2);
      satc_point_copy(point2, circle_pos);
      satc_point_sub(point2, points[prev]);
      region = satc_voronoi_region(edge, point2);
      if (region == SATC_RIGHT_VORONOI_REGION) {
        double dist = satc_point_len(point);
        if (dist > radius) {
          return false;
        } else if (response != NULL) {
          response->b_in_a = false;
          satc_point_normalize(point);
          overlap_n = point;
          overlap = radius - dist;
        }
      }
    } else if (region == SATC_RIGHT_VORONOI_REGION) {
      satc_point_copy(edge, polygon->edges[next]);
      satc_point_copy(point, circle_pos);
      satc_point_sub(point, points[next]);
      region = satc_voronoi_region(edge, point);
      if (region == SATC_LEFT_VORONOI_REGION) {
        double dist = satc_point_len(point);
        if (dist > radius) {
          return false;
        } else if (response != NULL) {
          response->b_in_a = false;
          satc_point_normalize(point);
          overlap_n = point;
          overlap = radius - dist;
        }
      }
    } else {
      satc_point_alloca(normal);
      satc_point_perp(edge);
      satc_point_normalize(edge);
      satc_point_copy(normal, edge);
      double dist = satc_point_dot(point, normal);
      double dist_abs = fabs(dist);
      if (dist > 0.0 && dist_abs > radius) {
        return false;
      } else {
        overlap_n = normal;
        overlap = radius - dist;
        if (dist >= 0.0 || overlap < 2.0 * radius) {
          response->b_in_a = false;
        }
      }
    }

    if (overlap_n != NULL && response != NULL && fabs(overlap) < fabs(response->overlap)) {
      response->overlap = overlap;
      satc_point_copy(response->overlap_n, overlap_n);
    }
  }

  if (response != NULL) {
    response->a = polygon;
    response->b = circle;
    satc_point_copy(response->overlap_v, response->overlap_n);
    satc_point_scale_x(response->overlap_v, response->overlap);
  }

  return true;
}

/**
 * Checks to see if one circle and one polygon are overlapping.
 *
 * This is ever so slightly more expensive than `satc_test_polygon_circle`,
 * since the response data needs to be flipped after checking collision.
 *
 * @param circle a circle.
 * @param polygon a polygon.
 * @param response the response object to set with collision data.
 */
bool satc_test_circle_polygon (satc_circle_t *circle, satc_polygon_t *polygon, satc_response_t *response) {
  bool result = satc_test_polygon_circle(polygon, circle, response);
  if (result && response != NULL) {
    void *a = response->a;
    bool a_in_b = response->a_in_b;
    satc_point_reverse(response->overlap_n);
    satc_point_reverse(response->overlap_v);
    response->a = response->b;
    response->b = a;
    response->a_in_b = response->b_in_a;
    response->b_in_a = a_in_b;
  }

  return result;
}

/**
 * Checks to see if one polygon and another are overlapping.
 *
 * @param a a polygon.
 * @param b another polygon.
 * @param response the response object to set with collision data.
 */
bool satc_test_polygon_polygon (satc_polygon_t *a, satc_polygon_t *b, satc_response_t *response) {
  double **a_points = a->points;
  size_t a_len = a->num_points;
  double **b_points = b->points;
  size_t b_len = b->num_points;

  size_t i = 0;
  for (; i < a_len; i++) {
    if (satc_is_separating_axis(a->pos, b->pos, a_len, a_points, b_len, b_points, a->normals[i], response)) {
      return false;
    }
  }

  i = 0;
  for (; i < b_len; i++) {
    if (satc_is_separating_axis(a->pos, b->pos, a_len, a_points, b_len, b_points, b->normals[i], response)) {
      return false;
    }
  }

  if (response != NULL) {
    response->a = a;
    response->b = b;
    satc_point_copy(response->overlap_v, response->overlap_n);
    satc_point_scale_x(response->overlap_v, response->overlap);
  }

  return true;
}

#endif
