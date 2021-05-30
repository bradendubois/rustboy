#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Mode0,  // HBlank Period
    Mode1,  // VBlank Period
    Mode2,  // Searching OAM Period
    Mode3   // Transferring Data to LCD Controller
}

/*
impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


 */