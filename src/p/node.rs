use crate::prelude::*;
use crate::Move;

use std::collections::HashSet;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct PrevInfo {
    node: NodeImpl,                   // parent node
    next_move: Move,                  // this node's previous move
    starting_comment: Option<String>, // Comment about starting a variation
    nag_set: HashSet<u8>,             // this node's nag attributes
}

#[derive(Debug, Clone)]
struct NodeInner {
    prev: Option<PrevInfo>,
    variation_vec: Vec<NodeImpl>,
    comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NodeImpl {
    inner: Rc<RefCell<NodeInner>>,
}

impl PartialEq<Self> for NodeImpl {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Default for NodeImpl {
    fn default() -> Self {
        let inner = NodeInner {
            prev: None,
            variation_vec: Vec::new(),
            comment: None,
        };
        Self::from_inner(inner)
    }
}

impl Node for NodeImpl {
    fn from_node(parent: Self, m: Move) -> Self {
        let inner = NodeInner {
            prev: Some(PrevInfo {
                node: parent,
                next_move: m,
                starting_comment: None,
                nag_set: HashSet::new(),
            }),
            variation_vec: Vec::new(),
            comment: None,
        };

        Self::from_inner(inner)
    }

    fn parent(&self) -> Option<Self> {
        Some(self.inner.borrow().prev.clone()?.node)
    }

    fn prev_move(&self) -> Option<Move> {
        Some(self.inner.borrow().prev.clone()?.next_move)
    }

    fn variation_vec(&self) -> Vec<Self> {
        self.inner.borrow().variation_vec.clone()
    }

    fn set_variation_vec(&mut self, new_variation_vec: Vec<Self>) -> Vec<Self> {
        std::mem::replace(
            &mut self.inner.borrow_mut().variation_vec,
            new_variation_vec,
        )
    }

    fn starting_comment(&self) -> Option<String> {
        self.inner.borrow().prev.clone()?.starting_comment
    }

    fn set_starting_comment(&mut self, new_comment: Option<String>) -> Option<String> {
        if let Some(ref mut prev) = self.inner.borrow_mut().prev {
            return std::mem::replace(&mut prev.starting_comment, new_comment);
        }

        None
    }

    fn nags(&self) -> Option<HashSet<u8>> {
        if let Some(ref prev) = self.inner.borrow().prev {
            return Some(prev.nag_set.iter().copied().collect());
        }

        None
    }

    fn set_nags(&mut self, new_nags: HashSet<u8>) -> Option<HashSet<u8>> {
        if let Some(ref mut prev) = self.inner.borrow_mut().prev {
            return Some(std::mem::replace(&mut prev.nag_set, new_nags));
        }

        None
    }

    fn comment(&self) -> Option<String> {
        self.inner.borrow().comment.clone()
    }

    fn set_comment(&self, new_comment: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.inner.borrow_mut().comment, new_comment)
    }
}

// Constructors
impl NodeImpl {
    fn from_inner(inner: NodeInner) -> Self {
        let inner = Rc::new(RefCell::new(inner));

        Self { inner }
    }
}
