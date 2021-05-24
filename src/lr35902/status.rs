use std::fmt;

pub enum Status {
    STOPPED,
    HALTED,
    RUNNING,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
