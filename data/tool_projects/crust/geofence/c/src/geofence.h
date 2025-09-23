#ifndef GEOFENCE_H
#define GEOFENCE_H

typedef struct point {
    double x;
    double y;
} point;

bool is_point_in_polygon(point *p_ptr, const point *vertices, size_t number_of_vertices);
bool is_position_in_geofence(point *p_ptr, const point *vertices, size_t number_of_vertices);
double lat2y_m(double lat); 
double lon2x_m(double lon);

#define polygon_size(p) (sizeof(p)/sizeof(point))

#endif