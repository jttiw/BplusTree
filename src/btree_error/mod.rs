use std::fmt;

pub struct BTreeError;

//TODO

impl fmt::Display for BTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "already in tree")
    }
}
impl fmt::Debug for BTreeError {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
}
}