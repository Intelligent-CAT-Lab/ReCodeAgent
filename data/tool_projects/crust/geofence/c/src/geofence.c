#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include "geofence.h"

#define DEG2RAD(a)   ((a) / (180 / M_PI))
#define RAD2DEG(a)   ((a) * (180 / M_PI))
#define EARTH_RADIUS 6378137

double lat2y_m(double lat) { 
    return log(tan( DEG2RAD(lat) / 2 + M_PI/4 )) * EARTH_RADIUS; 
}

double lon2x_m(double lon) { 
    return DEG2RAD(lon) * EARTH_RADIUS; 
}

bool is_position_in_geofence(point *p_ptr, const point *vertices, size_t number_of_vertices) {
    point xy[number_of_vertices];

    for (size_t i=0; i < number_of_vertices; i++) {
        xy[i].x = lon2x_m(vertices[i].x);
        xy[i].y = lat2y_m(vertices[i].y);        
    }

    point *p_m = (point*) malloc(sizeof(point));
    p_m->x = lon2x_m(p_ptr->x);
    p_m->y = lat2y_m(p_ptr->y);

    return is_point_in_polygon(p_m, xy, number_of_vertices);
}

bool is_point_in_polygon(point *p_ptr, const point *vertices, size_t number_of_vertices) {
    int number_of_intersections = 0;
    bool inside = false;
    double x = p_ptr->x;
    double y = p_ptr->y;

    if(number_of_vertices < 3){
        return false;
    } 

    for (size_t i=0; i < number_of_vertices; i++) {
        double x1 = vertices[i].x;
        double y1 = vertices[i].y;

        double x2 = vertices[(i + 1) % number_of_vertices].x;
        double y2 = vertices[(i + 1) % number_of_vertices].y;

        if ( (y1 > y && y2 > y) || (y1 < y && y2 < y) ){
            // No intersection
            continue;
        } else {
            // intersection of line segment from vertices and line segment from given point
            double s = x1 + (x2-x1) * ((y-y1)/(y2-y1));

            if(s <= x) {
                number_of_intersections++;
                if (x == x1 && y == y1) {
                    // if the point is at the vertex, then we need to reduce one count
                    number_of_intersections--;
                } else if (s == x1 && y == y1) {
                    // if the intersection is at a vertex, then we need to reduce one count
                    number_of_intersections--;
                }
            }
        }
    }

    return (number_of_intersections%2 == 1);
}
