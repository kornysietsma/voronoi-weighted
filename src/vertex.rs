#![warn(clippy::all)]
#![allow(dead_code)]
use super::face::Face;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
    // conflicts: ConflictList<Face>
    index: i32,
    handled: bool,
    // originalObject: thing to die in a fire
}
