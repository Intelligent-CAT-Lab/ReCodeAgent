use quadtree::quadtree::quadtree::*;
mod tests {
    use super::*;
    fn descent<T>(_node: &mut Option<Box<QuadtreeNode<T>>>) {}
    fn ascent<T>(_node: &mut Option<Box<QuadtreeNode<T>>>) {}
    #[test]
    fn test_node() {
        let mut node: QuadtreeNode<i32> = QuadtreeNode::quadtree_node_new();
        assert!(node.quadtree_node_isleaf());
        assert!(node.quadtree_node_isempty());
        assert!(node.quadtree_node_ispointer());
    }
    #[test]
    fn test_bounds() {
        let mut bounds = QuadtreeBounds::quadtree_bounds_new();
        assert!(bounds.nw.is_some());
        assert!(bounds.se.is_some());
        assert_eq!(bounds.nw.as_ref().unwrap().x, std::f64::INFINITY);
        assert_eq!(bounds.se.as_ref().unwrap().x, -std::f64::INFINITY);
        bounds.quadtree_bounds_extend( 5.0, 5.0);
        assert_eq!(bounds.nw.as_ref().unwrap().x, 5.0);
        assert_eq!(bounds.se.as_ref().unwrap().x, 5.0);
        bounds.quadtree_bounds_extend( 10.0, 10.0);
        assert_eq!(bounds.nw.as_ref().unwrap().y, 10.0);
        assert_eq!(bounds.se.as_ref().unwrap().y, 5.0);
        assert_eq!(bounds.width, 5.0);
        assert_eq!(bounds.height, 5.0);
        bounds.quadtree_bounds_free();
    }
    #[test]
    fn test_tree() {
        let mut val = Some(10);
        let mut tree: Quadtree<i32> = Quadtree::quadtree_new(1.0, 1.0, 10.0, 10.0);
        assert_eq!(tree.root.as_ref().unwrap().bounds.as_ref().unwrap().nw.as_ref().unwrap().x, 1.0);
        assert_eq!(tree.root.as_ref().unwrap().bounds.as_ref().unwrap().nw.as_ref().unwrap().y, 10.0);
        assert_eq!(tree.root.as_ref().unwrap().bounds.as_ref().unwrap().se.as_ref().unwrap().x, 10.0);
        assert_eq!(tree.root.as_ref().unwrap().bounds.as_ref().unwrap().se.as_ref().unwrap().y, 1.0);
        assert!(!tree.quadtree_insert(0.0, 0.0, val.clone()));
        assert!(!tree.quadtree_insert(110.0, 110.0, val.clone()));
        assert!(tree.quadtree_insert(8.0, 2.0, val.clone()));
        assert_eq!(tree.length, 1);
        assert_eq!(tree.root.as_ref().unwrap().point.as_ref().unwrap().x, 8.0);
        assert_eq!(tree.root.as_ref().unwrap().point.as_ref().unwrap().y, 2.0);
        assert!(!tree.quadtree_insert(0.0, 1.0, val.clone()));
        assert!(tree.quadtree_insert(2.0, 3.0, val.clone()));
        assert!(tree.quadtree_insert(2.0, 3.0, val.clone()));
        assert_eq!(tree.length, 2);
        assert!(tree.root.as_ref().unwrap().point.is_none());
        assert!(tree.quadtree_insert(3.0, 1.1, val.clone()));
        assert_eq!(tree.length, 3);
        assert_eq!(tree.quadtree_search(3.0, 1.1).as_ref().unwrap().x, 3.0);
        tree.quadtree_walk(ascent, descent);
        tree.quadtree_free();
    }
    #[test]
    fn test_points() {
        let mut point = QuadtreePoint::quadtree_point_new(5.0, 6.0);
        assert_eq!(point.x, 5.0);
        assert_eq!(point.y, 6.0);
        point.quadtree_point_free();
    }
}
fn main() {}