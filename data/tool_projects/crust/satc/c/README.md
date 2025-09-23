# satc

By Roger Jungemann

## What is it?

`satc` is a SAT (Separating Axis Theorem) collision detection library, written
in plain C99, as a single header file, with zero dependencies.

It is a port of [sat-js](https://github.com/jriecken/sat-js).

This library is readily usable for any 2D game purposes, although it only
handles overlap/collision, and is not a physics library. Friction, restitution,
drag, velocity, acceleration, etc. will need to be handled yourself if needed.
Still, it should be handy for a vast number of use cases.

It compiles with `-Wall` with no warnings, has 100% documentation coverage, and
nearly 100% test coverage.

It internally uses `alloca` for temporary variables, so they are allocated on
the stack, which is quicker than allocating on the heap, and values are
deallocated when they fall out of scope. Of course, any points, shapes, or
collision response objects you create will have to be deallocated. The docs
explain when you are responsible for deallocating a particular object.

Points are represented as arrays of doubles. I am considering allowing this to
be easily changed, but for now, it should handle nearly any use case.

Please feel free to file issues or pull requests.

## Usage

Simply place the `satc.h` file somewhere in your header search path, and
`#include "satc.h"` in your file.

To run the tests, from inside the `satc` directory, run `make test`.

To generate the docs, from inside the `satc` directory, run `make docs`. The
docs will be available at `html/index.html`.

## Example

The `satc-test.c` file has some good examples.

The following example illustrates polygon-to-circle collision:

```c
// Generate a circle.
double *c_pos = sc_point_alloca_xy(50.0, 50.0);
double c_radius = 20.0;
satc_circle_t *circle = satc_circle_create(c_pos, c_radius);

// Generate a polygon.
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
satc_polygon_t *polygon = satc_polygon_create(p_pos, 4, points);

// Generate a collision response object.
satc_response_t *response = satc_response_create();

// Check for collision.
bool collided = satc_test_polygon_circle(polygon, circle, response);

// Some explanation:
//
// * `collided` is `true` if the circle and polygon overlap.
// * `response->a` is the polygon.
// * `response->b` is the circle.
// * `response->overlap` is numerical amount of overlap.
// * `response->overlap_n` is the unit vector of the overlap.
// * `response->overlap_v` is the vector of the overlap.

// Deallocate the polygon.
satc_polygon_destroy(polygon);

// Deallocate the circle.
satc_circle_destroy(circle);
```

## API Docs

These will be added to `README.md`, but in the meantime, run `doxygen` to
generate the API docs, and visit `html/index.html` to view the API docs.
