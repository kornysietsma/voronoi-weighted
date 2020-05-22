#![warn(clippy::all)]
#![allow(dead_code)]
use super::face::Face;
use super::vertex::Vertex;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct HorizonEdge {
    origin: Rc<Vertex>,
    dest: Rc<Vertex>,
    twin: Rc<HorizonEdge>,
    i_face: Face,
    next: Rc<HorizonEdge>,
    prev: Rc<HorizonEdge>,
}
