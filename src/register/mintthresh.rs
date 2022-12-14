//! mintthresh register

use bit_field::BitField;

/// mintthresh register
#[derive(Clone, Copy, Debug)]
pub struct Mintthresh {
    bits: usize,
}

impl Mintthresh {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Gets the interrupt threshold
    #[inline]
    pub fn get_thresh(&self) -> usize {
        self.bits.get_bits(0..8)
    }

    /// Sets the interrupt threshold
    #[inline]
    pub fn set_thresh(&mut self, threshold:u8) -> () {
        self.bits.set_bits(0..8, threshold.into());
    }

    #[inline]
    pub fn new(bits:usize) -> Mintthresh{
        Mintthresh { bits: bits }
    }
}

read_csr_as!(Mintthresh, 0x347);

write_csr_as!(Mintthresh, 0x347);