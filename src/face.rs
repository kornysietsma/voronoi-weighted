#![warn(clippy::all)]
#![allow(dead_code)]

use super::edge::Edge;
use super::vector::Vector;
use super::vertex::Vertex;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Face {
    // conflicts: ConflictList<Vertex>
    marked: bool,
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
    normal: Vector, // index: int; - ONLY IN JAVA!
                    // dualPoint: Option<Point2d>;
}

// impl Face {
//     pub fn new(x: f64, y: f64, z: f64) -> Face {
//         let mut f = Face { x, y, z }
//     }
// }
