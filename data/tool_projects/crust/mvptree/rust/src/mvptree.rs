use std::fs::File;
use std::io::{self, Write};
use std::ptr;
use std::os::raw::c_int;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
pub const TAG: &str = "phashmvp2010";
pub const VERSION: u32 = 0x01000000;
pub const HEADER_SIZE: usize = 32;
pub const FILE_OFFSET_BITS: usize = 64;
pub const ERROR_MSGS: [&str; 25] = [
    "no error",
    "bad argument",
    "no distance function found",
    "mem alloc error",
    "no leaf node created",
    "no internal node created",
    "no path array alloc'd",
    "could not select vantage points",
    "could not calculate range from an sv1",
    "could not calculate range from an sv2",
    "points too compact",
    "could not sort points",
    "could not open file",
    "could not close file",
    "mmap error",
    "unmap error",
    "no write",
    "could not extend file",
    "could not remap file",
    "datatypes in conflict",
    "no. retrieved exceeds k",
    "empty tree",
    "distance value either NaN or less than zero",
    "could not open file",
    "unrecognized node",
];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MVPDataType {
    ByteArray = 1,
    UInt16Array = 2,
    UInt32Array = 4,
    UInt64Array = 8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    InternalNode = 1,
    LeafNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MVPError {
    Success,
    ArgErr,
    NoDistanceFunc,
    MemAlloc,
    NoLeaf,
    NoInternal,
    PathAlloc,
    VpNoSelect,
    NoSv1Range,
    NoSv2Range,
    NoSpace,
    NoSort,
    FileOpen,
    FileClose,
    MemMap,
    Munmap,
    NoWrite,
    FileTruncate,
    MremapFail,
    TypeMismatch,
    KNearestCap,
    EmptyTree,
    NoSplits,
    BadDistVal,
    FileNotFound,
    Unrecognized,
}
#[derive(Debug, Clone)]
pub struct MVPDatapoint {
    pub id: String,
    pub data: Vec<u8>,
    pub path: Vec<f32>,
    pub datalen: usize,
    pub data_type: MVPDataType,
}
pub type DistanceFunction = fn(&MVPDatapoint, &MVPDatapoint) -> f32;
pub struct InternalNode {
    pub node_type: NodeType,
    pub sv1: Option<Arc<MVPDatapoint>>,
    pub sv2: Option<Arc<MVPDatapoint>>,
    pub m1: Vec<f32>,
    pub m2: Vec<f32>,
    pub child_nodes: Vec<Rc<RefCell<Node>>>,
}
impl InternalNode{
    pub fn new(bf:u32) -> Self {
       unimplemented!() 
    }
}
pub struct LeafNode {
    pub node_type: NodeType,
    pub sv1: Option<Arc<MVPDatapoint>>,
    pub sv2: Option<Arc<MVPDatapoint>>,
    pub points: Vec<Arc<MVPDatapoint>>,
    pub d1: Vec<f32>,
    pub d2: Vec<f32>,
    pub nbpoints: usize,
}
impl LeafNode {
    pub fn new(bf:u32) -> Self {
        unimplemented!()
    }
}
pub enum Node {
    Leaf(LeafNode),
    Internal(InternalNode),
}
pub struct MVPTree {
    pub branch_factor: usize,
    pub path_length: usize,
    pub leaf_capacity: usize,
    pub datatype: MVPDataType,
    pub pos: i64,
    pub size: i64,
    pub pgsize: i64,
    pub buf: Vec<u8>,
    pub node: Option<Rc<RefCell<Node>>>,
    pub distance_function: DistanceFunction,
}
impl MVPTree {
    pub fn new(branch_factor: usize, path_length: usize, leaf_capacity: usize, datatype: MVPDataType, distance_function: DistanceFunction) -> Self {
        unimplemented!()
    }
    pub fn add(&mut self, points: Vec<MVPDatapoint>) -> MVPError {
        unimplemented!() 
    }
    pub fn retrieve(&self, target: &MVPDatapoint, knearest: usize, radius: f32) -> Result<Vec<MVPDatapoint>, MVPError> {
        unimplemented!()
    }
    pub fn write(&self, filename: &str, mode:i32) -> MVPError {
        unimplemented!()
    }
    pub fn print(&self, stream: &mut dyn Write) -> MVPError {
        unimplemented!()
    }
    pub fn clear(&mut self, node: &mut Option<Box<Node>>) {
        unimplemented!()
    }
    pub fn extend_mvpfile(&mut self)-> i32{
        unimplemented!()
    }
}
pub fn mvptree_read(filename: &str, distance_function: DistanceFunction) -> Result<MVPTree, MVPError> {
    unimplemented!()
}
impl MVPDatapoint {
    pub fn new(id: String, data: Vec<u8>, data_type: MVPDataType) -> Self {
        let datalen = data.len();
        MVPDatapoint {
            id,
            data,
            path: vec![],
            datalen,
            data_type,
        }
    }
    pub fn select_vantage_points(&mut self, nb:u32, sv1_pos: i32, sv2_pos: i32, dist: DistanceFunction) -> i32 {
        unimplemented!()
    }
    pub fn find_splits(&mut self, nb:u32, vp:&MVPDatapoint, tree: &MVPTree,  lengthM: u32) -> f32{
        unimplemented!()
    }
    pub fn sort_points(&mut self, nb:u32, sv1_pos: i32, sv2_pos: i32, vp: &MVPDatapoint, tree: &MVPTree, counts: &mut Vec<Vec<i32>>, pivots: Vec<f32>) -> Vec<Vec<Vec<Arc<MVPDatapoint>>>> {
        unimplemented!()
    }
    pub fn find_distance_range_for_vp(&mut self, nb:u32, vp: &MVPDatapoint, tree: &MVPTree, level: i32) -> i32 {
        unimplemented!()
    }
    pub fn write(&self, tree: &MVPTree) -> i64 {
        unimplemented!()
    }
}
pub fn error_to_string(error: MVPError) -> &'static str {
   unimplemented!()
}