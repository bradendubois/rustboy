pub trait Byte {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}


/// The traits that any memory bank controller (MBC) must implement
pub trait MBC {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

/// The GameBoy uses memory-mapped I/O; implementing a read / write trait ensures consistency
/// across various modules that will utilize this concept, as well as clarify the meaning of
/// a "read" or "write" in relation to a specific module.
///
/// The GameBoy uses a 16-bit address space, while all values in memory / hardware that can be
/// stored are always 8-bits. 8-bit CPU registers can be combined (such as with the HL pair) to
/// form a single 16-bit bit value that can be used as an address.
pub trait MemoryMap {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

