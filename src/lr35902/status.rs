#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub enum Status {
    STOPPED,
    HALTED,
    RUNNING,
}
