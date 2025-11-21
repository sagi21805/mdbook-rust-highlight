use crate::tokens::Tag;
use std::collections::HashMap;

pub enum SearchStep {
    Next(usize),
    Tag(Tag),
}

pub type Node = HashMap<&'static str, SearchStep>;

pub struct PathTracer(pub Vec<Node>);

impl PathTracer {
    pub fn new() -> Self {
        Self(vec![Node::new()])
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

            let maybe_new = match self.0[current_idx].get(seg_str.as_str()) {
                Some(a) => match a {
                    SearchStep::Next(next) => Some(*next),
                    SearchStep::Tag(_) => unreachable!(),
                },
                None => None,
            };

            let new = if let Some(new) = maybe_new {
                new
            } else {
                let new = self.0.len();
                self.0.push(Node::new());

                self.0[new].insert(seg_str.leak(), SearchStep::Next(new));
                new
            };
            current_idx = new;
        }

        let n = &mut self.0[current_idx];

        n.insert(last.ident.to_string().leak(), SearchStep::Tag(tag));
    }

    pub fn get(&self, p: &syn::Path) -> Option<Tag> {
        let mut current_idx = 0;

        for seg in &p.segments {
            let seg_str = seg.ident.to_string();

            match self.0[current_idx].get(seg_str.as_str())? {
                SearchStep::Next(next) => current_idx = *next,
                SearchStep::Tag(tag) => return Some(tag.clone()),
            }
        }
        None
    }
}
