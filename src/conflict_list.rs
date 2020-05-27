#![warn(clippy::all)]
#![allow(dead_code)]
use super::face::Face;
use super::vertex::Vertex;
use std::marker;
use std::marker::PhantomData;
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

#[derive(Debug, PartialEq)]
pub struct ConflictList<'a, T> {
    the_head: Option<Rc<ConflictListNode<'a>>>,
    _marker: marker::PhantomData<&'a T>,
}

impl<'a, T> ConflictList<'a, T> {
    pub fn new() -> Self {
        ConflictList {
            the_head: None,
            _marker: PhantomData,
        }
    }

    fn empty(&self) -> bool {
        self.the_head.is_none()
    }

    pub fn get_vertices(&mut self) -> Vec<&Vertex> {
        let mut results: Vec<&Vertex> = Vec::new();
        let mut curr = &self.the_head;
        while let Some(h) = curr {
            results.push(&h.vertex);
            curr = &h.nextv;
        }
        results
    }
}

impl<'a> ConflictList<'a, Face> {
    pub fn add(&mut self, node: &mut &'a Rc<ConflictListNode>) {
        // new node needs to be mutable
        // probably need a RefCell
        // see file:///Users/korny/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch15-05-interior-mutability.html
        // let mut new_node = Rc::clone(node);
        match &self.the_head {
            None => self.the_head = Some(Rc::clone(node)),
            Some(head) => {
                let mut new_node = Rc::clone(node);
                new_node.nextv = Some(Rc::clone(head));
                head.prevv = Some(new_node);
                self.the_head = Some(new_node);
            }
        }
    }

    /// Remove all vertices from Face
    pub fn removeAll(&mut self) {
        let mut curr_opt = self.the_head;
        while let Some(curr) = curr_opt {
            if let Some(prevf) = curr.prevf {
                // Node is not head of a face list
                if let Some(nextf) = curr.nextf {
                    // unlink current node from next face if it has one
                    nextf.prevf = curr.prevf;
                }
                // unlink previous face's link to this node
                prevf.nextf = curr.nextf
            } else {
                // Node is head of a face list
                if let Some(nextf) = curr.nextf {
                    // unlink this from the next face if there was one
                    nextf.prevf = None;
                // TODO: fix this:
                // update the head reference in the corresponding Vertex's conflict list
                // curr.vertex.getList().head = nextf
                } else {
                    // TODO: fix this:
                    // remove the head reference from the corresponding Vertex's conflict list
                    // curr.vertex.getList().head = None
                }
            }
            curr_opt = curr.nextv; // traverse the list of vertexes
            if let Some(nextcurr) = curr_opt {
                // unlink the next node's previous vertex link - should be the final decoupling
                nextcurr.prevv = None;
            }
        }
    }
}

impl<'a> ConflictList<'a, Vertex> {
    pub fn add(&mut self, node: &'a Rc<ConflictListNode>) {
        let new_node = Rc::clone(node);
        match self.the_head {
            None => self.the_head = Some(new_node),
            Some(head) => {
                head.prevf = Some(new_node);
                new_node.nextf = Some(head);
                self.the_head = Some(new_node);
            }
        }
    }

    /// Returns visible list of all visible faces of the vertex
    /// and also marks the faces.
    /// note this was 'fill' in original code
    ///  and really could do with marking some other way?
    ///   as we can't get the face mutably without pain.
    pub fn get_visible_faces_and_mark(&mut self) -> Vec<&Face> {
        let results: Vec<&Face> = Vec::new();
        let mut curr = self.the_head;
        while let Some(h) = curr {
            // TODO: fix this
            // h.face.setMarked(true);
            results.push(h.face);
            curr = h.nextf;
        }
        results
    }

    /// Remove all faces from vertex
    /// identical to Face implementation but using the other linked lists
    pub fn removeAll(&mut self) {
        let mut curr_opt = self.the_head;
        while let Some(curr) = curr_opt {
            if let Some(prevv) = curr.prevv {
                // Node is not head of a vertex list
                if let Some(nextv) = curr.nextv {
                    // unlink current node from next vertex if it has one
                    nextv.prevv = curr.prevv;
                }
                // unlink previous vertex's link to this node
                prevv.nextv = curr.nextv
            } else {
                // Node is head of a vertex list
                if let Some(nextv) = curr.nextv {
                    // unlink this from the next vertex if there was one
                    nextv.prevv = None;
                // TODO: fix this:
                // update the head reference in the corresponding Faces's conflict list
                // curr.face.getList().head = nextv
                } else {
                    // TODO: fix this:
                    // remove the head reference from the corresponding Face's conflict list
                    // curr.face.getList().head = None
                }
            }
            curr_opt = curr.nextf; // traverse the list of faces
            if let Some(nextcurr) = curr_opt {
                // unlink the next node's previous face link - should be the final decoupling
                nextcurr.prevf = None;
            }
        }
    }
}
