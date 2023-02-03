//! mcountinhibit register

use bit_field::BitField;

/// mcounteren register
#[derive(Clone, Copy, Debug)]
pub struct Mcountinhibit {
    bits: usize,
}

impl Mcountinhibit {
    /// Cycle count disable
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }

    ///  Insret count disable
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Supervisor "hpm\[x\]" Count Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!(3 <= index && index < 32);
        self.bits.get_bit(index)
    }
}

read_csr_as!(Mcountinhibit, 0x320);
write_csr!(0x320);
set!(0x320);
clear!(0x320);

set_clear_csr!(
/// Cycle count Disable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// Instret count Disable
    , set_ir, clear_ir, 1 << 2);

#[inline]
pub unsafe fn set_hpm(index: usize) {
    assert!(3 <= index && index < 32);
    _set(1 << index);
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!(3 <= index && index < 32);
    _clear(1 << index);
}
