use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
pub struct DataPoint{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub idx: usize,
}
pub struct TreeNode{
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub split: f64,
    pub idx: usize,
}
pub struct Boundaries{
    pub min: f64,
    pub max: f64,
}
pub struct space{
    pub dim: [Boundaries; 3]
}
pub struct KDTree{
    pub count: usize,
    pub max_nodes: usize,
    pub next_node: usize,
    pub points: Vec<DataPoint>,
    pub node_data: Vec<Rc<RefCell<TreeNode>>>,
    pub root: Option<Rc<RefCell<TreeNode>>>,
}
impl KDTree{
    pub fn new() -> Self {
        unimplemented!();
    }
    pub fn build(&mut self, x: &mut [f64], y: &mut [f64], z: &mut [f64], count: usize) {
        unimplemented!();
    }
    pub fn search(&self, iter: &mut Option<KDTreeIterator>,x: f64, y: f64, z: f64, apothem: f64) {
        unimplemented!();
    }
    pub fn search_space(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) {
        unimplemented!();
    }
    pub fn delete(&mut self) {
        unimplemented!();
    }
    fn next_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        unimplemented!();
    }
    fn get_branch_node(&mut self, split: f64) -> Option<Rc<RefCell<TreeNode>>> {
        unimplemented!();
    }
    fn is_leaf(&self, node: &Rc<RefCell<TreeNode>>) -> i32 {
        unimplemented!();
    }
    fn point_in_search_space(&self, point: &DataPoint, search_space: &space) -> i32 {
        unimplemented!();
    }
    fn completely_enclosed(&self, search_space: &space, domain: &space) -> i32 {
        unimplemented!();
    }
    fn search_area_intersects(&self, search_space: &space, domain: &space) -> i32 {
        unimplemented!();
    }
    fn report_all_leaves(&self, node: &Rc<RefCell<TreeNode>>, iter: &KDTreeIterator){
        unimplemented!();
    }
    fn explore_branch(&self, node: &Rc<RefCell<TreeNode>>, depth: usize, search_space: &space, domain: &space, iter: &KDTreeIterator){
        unimplemented!();
    }
    fn search_kd(&self, root: &Rc<RefCell<TreeNode>>,  depth: usize, search_space: &space, domain: &space, iter: &KDTreeIterator){
        unimplemented!();
    }
    fn build_kdtree(&mut self, points: &mut [DataPoint], depth: usize) {
        unimplemented!();
    }
}
pub struct KDTreeIterator{
    pub data: Vec<usize>,
    pub capacity: usize,
    pub size: usize,
    pub current: usize,
}
impl KDTreeIterator{
    pub fn new() -> Self {
        unimplemented!();
    }
    pub fn reset(&mut self) {
        unimplemented!();
    }
    pub fn push(&mut self, value: usize) {
        unimplemented!();
    }
    pub fn get_next(&mut self) -> Option<usize> {
        unimplemented!();
    }
    fn rewind(&mut self) {
        unimplemented!();
    }
    fn delete(&mut self) {
        unimplemented!();
    }
    fn sort(&mut self) {
        unimplemented!();
    }
}
fn compare_x(a: &DataPoint, b: &DataPoint) -> std::cmp::Ordering {
    unimplemented!();
}
fn compare_y(a: &DataPoint, b: &DataPoint) -> std::cmp::Ordering {
    unimplemented!();
}
fn compare_z(a: &DataPoint, b: &DataPoint) -> std::cmp::Ordering {
    unimplemented!();
}
fn compare_size_t(a: &usize, b: &usize) -> std::cmp::Ordering {
    unimplemented!();
}