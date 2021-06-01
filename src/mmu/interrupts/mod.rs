pub mod interrupt_enable;
pub mod interrupt_flag;

pub trait InterruptGenerator {
    fn disable_interrupt_generation(&mut self);
    fn enable_interrupt_generation(&mut self);
    fn interrupt_u8(&mut self) -> u8;
    fn clear_interrupt_bit(&mut self, bit: u8);
}
