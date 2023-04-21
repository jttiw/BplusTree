use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct LeafElement<T: Ord, V: Sized> {
    pub comp: T,
    pub data: V
}

//Constructor + Getters and Setters

impl <T: Ord, V: Sized> LeafElement<T, V> {
    pub fn new(comp: T, data: V) -> Self {
        LeafElement {
            comp,
            data
        }
    }

}