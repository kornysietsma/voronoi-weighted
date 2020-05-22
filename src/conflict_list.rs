#![warn(clippy::all)]
#![allow(dead_code)]
use super::face::Face;
use super::vertex::Vertex;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ConflictListNode<'a> {
    face: &'a Face,
    vertex: Rc<Vertex>,
    nextf: Option<Rc<ConflictListNode<'a>>>,
    prevf: Option<Rc<ConflictListNode<'a>>>,
    nextv: Option<Rc<ConflictListNode<'a>>>,
    prevv: Option<Rc<ConflictListNode<'a>>>,
}

impl<'a> ConflictListNode<'a> {
    pub fn new(face: &'a Face, vertex: Vertex) -> Self {
        ConflictListNode {
            face: &face,
            vertex: Rc::new(vertex),
            nextf: None,
            prevf: None,
            nextv: None,
            prevv: None,
        }
    }
}

pub trait ConflictList {
    fn head(&self) -> Option<Rc<ConflictListNode>>;
}

#[derive(Debug, PartialEq)]
pub struct FaceConflictList<'a> {
    the_head: Option<Rc<ConflictListNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct VertexConflictList<'a> {
    the_head: Option<Rc<ConflictListNode<'a>>>,
}

impl<'a> ConflictList for FaceConflictList<'a> {
    fn head(&self) -> Option<Rc<ConflictListNode>> {
        self.the_head.map(|h| Rc::clone(&h))
    }
}

impl<'a> ConflictList for VertexConflictList<'a> {
    fn head(&self) -> Option<Rc<ConflictListNode>> {
        self.the_head.map(|h| Rc::clone(&h))
    }
}

impl<'a> FaceConflictList<'a> {
    pub fn new() -> Self {
        FaceConflictList { the_head: None }
    }

    pub fn add(&mut self, node: &'a Rc<ConflictListNode>) {
        let new_node = Rc::clone(node);
        match self.the_head {
            None => self.the_head = Some(new_node),
            Some(head) => {
                head.prevv = Some(new_node);
                new_node.nextv = Some(head);
                self.the_head = Some(new_node);
            }
        }
    }
}

impl<'a> VertexConflictList<'a> {
    pub fn new() -> Self {
        VertexConflictList { the_head: None }
    }
}
