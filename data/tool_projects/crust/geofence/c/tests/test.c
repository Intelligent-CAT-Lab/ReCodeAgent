#include <stdio.h>
#include <stdbool.h>
#include "geofence.h"

typedef bool(*geofence_func)(point*, const point*, size_t);

typedef struct testcase {
    point *point;
    point *vertices;
    size_t number_of_vertices;
    bool inside;
} testcase;

point origin = {
    .x = 0,
    .y = 0
};

point p2 = {
    .x = -1,
    .y = 0
};

point p3 = {
    .x = 0,
    .y = 0.5
};

point p4 = {
    .x = 5,
    .y = 0.5
};

point p5 = {
    .x = 0.5,
    .y = 0
};

point p6 = {
    .x = -1,
    .y = 1
};

point poly1[] = {
    {
        .x = 1,
        .y = 1
    },
    {
        .x = 1,
        .y = -1
    },
    {
        .x = -1,
        .y = -1
    },
    {
        .x = -1,
        .y = 1
    }
};

point poly2[] = {
    {
        .x = 0,
        .y = 1
    },
    {
        .x = 1,
        .y = 0
    },
    {
        .x = 0,
        .y = -1
    },
    {
        .x = -1,
        .y = 0
    }
};

point poly3[] = {
    {
        .x = 0,
        .y = 1
    },
    {
        .x = 1,
        .y = 0
    },
    {
        .x = 0,
        .y = -1
    },
    {
        .x = -1,
        .y = 0
    },
    {
        .x = 0,
        .y = 0
    }
};


testcase poly_testcases[] = {
    {&origin, poly1, polygon_size(poly1), true},
    {&p2, poly1, polygon_size(poly1), true},
    {&p3, poly2, polygon_size(poly2), true},
    {&p4, poly2, polygon_size(poly2), false},
    {&origin, poly2, polygon_size(poly2), true},
    {&p2, poly2, polygon_size(poly2), true},
    {&p3, poly3, polygon_size(poly3), true},
    {&origin, poly3, polygon_size(poly3), true},
    {&p5, poly3, polygon_size(poly3), true},
    {&p6, poly3, polygon_size(poly3), false}
};

point geofence_point1 = {
    .x = 77.64664,
    .y = 12.90383
};

point geofence_poly1[] = {
    {
        .x = 77.64354,
        .y = 12.90448
    },
    {
        .x = 77.64671,
        .y = 12.90573
    },
    {
        .x = 77.64815,
        .y = 12.90367
    },
    {
        .x = 77.64703,
        .y = 12.90215
    }
};

point geofence_point2 = {
    .x = 77.6183653,
    .y = 12.9086164
};

point geofence_point3 = {
    .x = 77.6168203, 
    .y = 12.9081563
};

point geofence_poly2[] = {
    {
        .x = 77.6084948,
        .y = 12.9115446
    },
    {
        .x = 77.6163054,
        .y = 12.9114191
    },
    {
        .x = 77.6179361,
        .y = 12.907236
    },
    {
        .x = 77.6226139,
        .y = 12.9099132
    },
    {
        .x = 77.6197386,
        .y = 12.9190739
    },
    {
        .x = 77.6300812,
        .y = 12.9186138
    },
    {
        .x = 77.6333857,
        .y = 12.9106243
    },
    {
        .x = 77.6314974,
        .y = 12.9017561
    },
    {
        .x = 77.6213264,
        .y = 12.9001665
    },
    {
        .x = 77.6100397,
        .y = 12.9019652
    }
};

testcase geofence_testcases[] = {
    {&geofence_point1, geofence_poly1, polygon_size(geofence_poly1), true},
    {&geofence_point2, geofence_poly2, polygon_size(geofence_poly2), false},
    {&geofence_point3, geofence_poly2, polygon_size(geofence_poly2), true}
};

void run_test(const testcase *testcases, size_t num_testcases, geofence_func func) {
    for (size_t i = 0; i < num_testcases; i++){
        printf("Test %ld: ", i + 1);
        testcase t = testcases[i];

        bool response = func(t.point, t.vertices, t.number_of_vertices);

        if(response == t.inside) {
            printf("Pass\n");
        } else {
            printf("Fail\n");
            printf("\tPoint: (%.8f, %.8f)\n", t.point->x, t.point->y);
            printf("\tPolygon:\n");

            for(size_t i = 0; i < t.number_of_vertices; i++){
                point p = (t.vertices)[i];
                printf("\t\t(%.8f, %.8f)\n", p.x, p.y);
            }
            
            printf("\tExpected answer: %s\n", t.inside? "true":"false");
            printf("\tReceived answer: %s\n", response? "true":"false");
        }
    }
}

int main() {
    printf("====== Polygon Tests ======\n");
    run_test(poly_testcases, sizeof(poly_testcases)/sizeof(poly_testcases[0]), is_point_in_polygon);
    
    printf("====== Geofence Tests ======\n");
    run_test(geofence_testcases, sizeof(geofence_testcases)/sizeof(geofence_testcases[0]), is_position_in_geofence);
    
    return 0;
}
