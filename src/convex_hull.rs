#![warn(clippy::all)]
#![allow(dead_code)]
use super::face::Face;
use super::horizon_edge::HorizonEdge;
use super::vertex::Vertex;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ConvexHull {
    points: Vec<Vertex>,
    facets: Vec<Face>,
    created: Vec<Face>,
    horizon: Vec<HorizonEdge>,
    visible: Vec<Face>,
}
