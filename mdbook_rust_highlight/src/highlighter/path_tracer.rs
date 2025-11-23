use crate::tokens::Tag;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
pub struct PathNode {
    pub next: Option<usize>,
    pub tag: Tag,
}

pub type Node = HashMap<&'static str, PathNode>;

#[derive(Debug, Clone)]
pub struct PathTracer {
    pub manual: HashMap<&'static str, Tag>,
    pub learned: Vec<Node>,
}

impl PathTracer {
    pub fn new() -> Self {
        Self {
            manual: HashMap::new(),
            learned: vec![Node::new()],
        }
    }

    pub fn map(&mut self, p: &syn::Path, tag: Tag) {
        let mut current_idx = 0;

        let last = p
            .segments
            .last()
            .expect("Path is empty and has no last element");

        let path_iter = p.segments.iter().take(p.segments.len() - 1);

        for seg in path_iter {
            let seg_str = seg.ident.to_string();

            let key = seg_str.as_str();

            if let Some(next) = self.learned[current_idx].get(key).and_then(|n| n.next) {
                current_idx = next;
            } else {
                let new = self.learned.len();
                self.learned.push(Node::new());
                match self.learned[current_idx].get_mut(key) {
                    Some(node) => {
                        node.next = Some(new);
                    }
                    None => {
                        self.learned[current_idx].insert(
                            seg_str.leak(),
                            PathNode {
                                next: Some(new),
                                tag: Tag::Type,
                            },
                        );
                    }
                }

                current_idx = new;
            }
        }

        let n = &mut self.learned[current_idx];
        n.entry(last.ident.to_string().leak())
            .or_insert(PathNode { next: None, tag });
    }

    pub fn get(&self, p: &syn::Path) -> Option<Tag> {
        let mut current_idx = 0;

        for seg in &p.segments {
            let seg_str = seg.ident.to_string();

            let node = self.learned[current_idx].get(seg_str.as_str())?;
            match node.next {
                Some(next) => current_idx = next,
                None => return Some(node.tag),
            }
        }
        None
    }
}
