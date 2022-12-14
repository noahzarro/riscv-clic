//! mtvt register

use bit_field::BitField;

/// mtvt register
#[derive(Clone, Copy, Debug)]
pub struct Mtvt {
    bits: usize,
}

impl Mtvt {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Gets the clic interrupt vector base
    #[inline]
    pub fn get_base(&self) -> usize {
        self.bits.get_bits()
    }

    /// Sets the clic interrupt vector base
    #[inline]
    pub fn set_base(&mut self, base:usize) -> () {
        self.bits.set_bits(base);
    }

    #[inline]
    pub fn new(bits:usize) -> Mtvt{
        Mtvt { bits: bits }
    }
}

read_csr_as!(Mtvt, 0x307);

write_csr_as!(Mtvt, 0x307);

/// Writes the CSR
#[inline]
pub unsafe fn write_addr(addr: usize) {
    let bits = addr;
    _write(bits);
}
