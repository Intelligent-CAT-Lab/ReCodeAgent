use std::fs::File;
use std::io::{Write, Read};
use std::sync::Arc;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f32;
use std::cmp;
use mvptree::mvptree::{MVPTree, MVPDatapoint, MVPDataType, MVPError};
use mvptree::mvptree::mvptree_read;
const MVP_BRANCHFACTOR: usize = 2;
const MVP_PATHLENGTH: usize = 5;
const MVP_LEAFCAP: usize = 25;
static mut NBCALCS: u64 = 0;
fn next_poisson_int(lambda: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut l = (-lambda as f64).exp();
    let mut p = 1.0;
    let mut k = 0;
    while p > l {
        k += 1;
        p *= rng.r#gen::<f64>();
    }
    k - 1
}
fn point_l1_distance(point_a: &MVPDatapoint, point_b: &MVPDatapoint) -> f32 {
    if point_a.data.len() != point_b.data.len() {
        return -1.0;
    }
    let sum: u32 = point_a.data.iter()
        .zip(&point_b.data)
        .map(|(a, b)| (*a as i32 - *b as i32).abs() as u32)
        .sum();
    unsafe { NBCALCS += 1; }
    sum as f32 / point_a.data.len() as f32
}
fn point_l2_distance(point_a: &MVPDatapoint, point_b: &MVPDatapoint) -> f32 {
    if point_a.data.len() != point_b.data.len() {
        return -1.0;
    }
    let sum: i32 = point_a.data.iter()
        .zip(&point_b.data)
        .map(|(a, b)| (*a as i32 - *b as i32).pow(2))
        .sum();
    unsafe { NBCALCS += 1; }
    (sum as f32).sqrt() / point_a.data.len() as f32
}
fn generate_point(dp_length: usize) -> MVPDatapoint {
    let mut rng = StdRng::seed_from_u64(98293928);
    let id = format!("point{}", rng.r#gen::<u64>());
    let data: Vec<u8> = (0..dp_length).map(|_| rng.r#gen()).collect();
    MVPDatapoint::new(id, data, MVPDataType::ByteArray)
}
fn generate_uniform_points(nbpoints: usize, dp_length: usize) -> Vec<MVPDatapoint> {
    (0..nbpoints).map(|_| generate_point(dp_length)).collect()
}
fn generate_cluster(nbpoints: usize, dplength: usize, var: i32, dist: fn(&MVPDatapoint, &MVPDatapoint) -> f32) -> Vec<MVPDatapoint> {
    let mut points = Vec::with_capacity(nbpoints);
    points.push(generate_point(dplength));
    let orig_data = points[0].data.clone();
    let mut rng = rand::thread_rng();
    let mut max_distance = 0.0;
    for _ in 1..nbpoints {
        let mut new_data = orig_data.clone();
        for val in &mut new_data {
            let diff = next_poisson_int(var);
            let toss: f32 = rng.r#gen();
            let mut v = if toss > 0.5 { diff } else { -diff };
            v += *val as i32;
            *val = cmp::min(255, cmp::max(0, v)) as u8;
        }
        let new_point = MVPDatapoint { id: format!("point{}", rng.r#gen::<u64>()), data: new_data, path: vec![], datalen: dplength, data_type: MVPDataType::ByteArray };
        let d = dist(&points[0], &new_point);
        if d > max_distance {
            max_distance = d;
        }
        points.push(new_point);
    }
    println!("Cluster - maximum distance from main point: {}", max_distance);
    points
}
#[test]
fn test() {
    let nbpoints = 100;
    let radius: f32 = 5.0;
    let nbcluster1 = nbpoints / 10;
    let knearest = nbpoints;
    let dplength = 10;
    let var = 10;
    let testfile = "testfile.mvp";
    let distance_func = point_l2_distance;
    let pointlist = generate_uniform_points(nbpoints, dplength);
    assert!(pointlist.len() == nbpoints);
    let cluster1 = generate_cluster(nbcluster1, dplength, var, distance_func);
    assert!(cluster1.len() == nbcluster1);
    let cluster_center = cluster1[0].clone();
    let mut tree = MVPTree::new(MVP_BRANCHFACTOR, MVP_PATHLENGTH, MVP_LEAFCAP, MVPDataType::ByteArray, distance_func);
    println!("Adding {} points to tree.", nbpoints);
    let mut error: MVPError = tree.add(pointlist);
    assert!(error == MVPError::Success);
    println!("Adding {} cluster points to tree.", nbcluster1);
    let mut error: MVPError = tree.add(cluster1);
    assert!(error == MVPError::Success);
    println!("Writing tree to file: {}", testfile);
    let mut error: MVPError = tree.write(testfile, 00755);
    assert!(error == MVPError::Success);
    let mut tree = mvptree_read(testfile, distance_func).unwrap();
    let mut savedpoint: Option<MVPDatapoint> = None;
    for count in 0..(nbpoints / 10) {
        let pnt = generate_point(dplength);
        if count == 0 {
            savedpoint = Some(pnt.clone());
        }
        println!("Adding point: {} to tree.", pnt.id);
        assert!(tree.add(vec![pnt]) == MVPError::Success);
    }
    println!("Retrieving point: {}", cluster_center.id);
    let results = tree.retrieve(&cluster_center, knearest, radius).expect("Failed to retrieve points");
    for (i, res) in results.iter().enumerate() {
        println!("  FOUND --> ({}) {}", i, res.id);
    }
    if let Some(savedpoint) = savedpoint {
        println!("Retrieving point: {}", savedpoint.id);
        let results = tree.retrieve(&savedpoint, knearest, radius).expect("Failed to retrieve points");
        for (i, res) in results.iter().enumerate() {
            println!("  FOUND --> ({}) {}", i, res.id);
        }
    }
    println!("Done.");
}
pub fn main() {}