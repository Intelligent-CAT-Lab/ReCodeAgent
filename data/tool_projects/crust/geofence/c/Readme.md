# Geofence
There are 2 functions defined in this project:
```c
bool is_point_in_polygon(point *p_ptr, const point *vertices, size_t number_of_vertices);
bool is_position_in_geofence(point *p_ptr, const point *vertices, size_t number_of_vertices);
```
`is_point_in_polygon` checks if a given point is inside a given polygon or not. It accepts input in terms of coordinates. 

`is_position_in_geofence` checks if a given location is inside a given geofence or not. It accepts input in terms of lng, lat. It performs a mercator conversion internally.