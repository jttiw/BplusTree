use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize,Clone, Copy)]
pub struct InnerElement<T: Ord> {
    pub comp: T, // wegbeschreibend
    pub id: usize  // id des nächsten knoten
}

impl <T: Ord> InnerElement<T> {
    pub fn new(comp: T, id: usize) -> Self {
        InnerElement {
            comp,
            id
        }
    }
}