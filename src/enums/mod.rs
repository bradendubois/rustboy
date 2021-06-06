#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub enum Status {
    STOPPED,
    HALTED,
    RUNNING,
}


#[derive(PartialEq)]
pub enum IME {
    Enabled,
    OneCycleDelay,
    ReadyToEnable,
    Disabled
}


#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2,  // Searching OAM Period
    Mode3   // Transferring Data to LCD Controller
}
