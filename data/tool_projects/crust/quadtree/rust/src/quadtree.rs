pub mod quadtree {
    #[derive(Default)]
    pub struct QuadtreePoint {
        pub x: f64,
        pub y: f64,
    }
    impl QuadtreePoint {
        pub fn quadtree_point_new(x: f64, y: f64) -> QuadtreePoint {
            unimplemented!()
        }
        pub fn quadtree_point_free(&self) {
            unimplemented!()
        } 
    }
    #[derive(Default)]
    pub struct QuadtreeBounds {
        pub nw: Option<Box<QuadtreePoint>>,
        pub se: Option<Box<QuadtreePoint>>,
        pub width: f64,
        pub height: f64,
    }
    impl QuadtreeBounds {
        pub fn quadtree_bounds_new() -> QuadtreeBounds {
            unimplemented!()
        }
        pub fn quadtree_bounds_extend(&self, x: f64, y: f64) {
            unimplemented!()
        }
        pub fn quadtree_bounds_free(&self) {
            unimplemented!()
        }
    }
    #[derive(Default)]
    pub struct QuadtreeNode<T> {
        pub ne: Option<Box<QuadtreeNode<T>>>,
        pub nw: Option<Box<QuadtreeNode<T>>>,
        pub se: Option<Box<QuadtreeNode<T>>>,
        pub sw: Option<Box<QuadtreeNode<T>>>,
        pub bounds: Option<Box<QuadtreeBounds>>,
        pub point: Option<Box<QuadtreePoint>>,
        pub key: Option<T>,
    }
    impl <T> QuadtreeNode<T> {
        pub fn node_contains_(&mut self, point: Option<Box<QuadtreePoint>>) {
            unimplemented!()
        }
        pub fn get_quadrant_(&mut self, point: Option<Box<QuadtreePoint>>) {
            unimplemented!()
        }
        pub fn find_(&mut self, x: f64, y: f64) {
            unimplemented!()
        }
        pub fn quadtree_node_new() -> QuadtreeNode<T> {
            unimplemented!()
        }
        pub fn quadtree_node_free(&self, value_free: Option<fn(Option<T>)>,) {
            unimplemented!()
        }
        pub fn quadtree_node_ispointer(&self) -> bool {
            unimplemented!()
        }
        pub fn quadtree_node_isempty(&self) -> bool {
            unimplemented!()
        }
        pub fn quadtree_node_isleaf(&self) -> bool {
            unimplemented!()
        }
        pub fn quadtree_node_reset(&self, value_free: Option<fn(Option<T>)>,) {
            unimplemented!()
        }
        pub fn quadtree_node_with_bounds(minx: f64, miny: f64, maxx: f64, maxy: f64) -> QuadtreeNode<T> {
            unimplemented!()
        }
    }
    #[derive(Default)]
    pub struct Quadtree<T> {
        pub root: Option<Box<QuadtreeNode<T>>>,
        pub key_free: Option<fn(Option<T>)>,
        pub length: u32,
    }
    impl <T> Quadtree<T> {
        pub fn split_node_(&mut self, node: Option<Box<QuadtreeNode<T>>>) {
            unimplemented!()
        }
        pub fn insert_(&mut self, tree: Option<Box<QuadtreeNode<T>>>, point: Option<Box<QuadtreePoint>>, key: Option<T>) {
            unimplemented!()
        }
        pub fn quadtree_new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Quadtree<T> {
            unimplemented!()
        }
        pub fn quadtree_free(&mut self) {
            unimplemented!()
        }
        pub fn quadtree_search(&self, x: f64, y: f64) -> &mut Option<Box<QuadtreePoint>> {
            unimplemented!()
        }
        pub fn quadtree_insert(&self, x: f64, y: f64, key: Option<T>) -> bool {
            unimplemented!()
        }
        pub fn quadtree_walk(&self, descent: fn(&mut Option<Box<QuadtreeNode<T>>>), ascent: fn(&mut Option<Box<QuadtreeNode<T>>>)) {
            unimplemented!()
        }
    }
}
// Helper function
pub fn elision_<T>(key: Option<Box<T>>) {unimplemented!()}