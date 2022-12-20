//! mintstatus register

/// mintstatus read only register
#[derive(Clone, Copy, Debug)]
pub struct Mintstatus {
    bits: usize,
}

impl Mintstatus {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the machine mode interrupt level
    #[inline]
    pub fn mil(&self) -> usize {
        (self.bits >> 24) & 0xFF
    }

    /// Returns the supervisor mode interrupt level
    #[inline]
    pub fn sil(&self) -> usize {
        (self.bits >> 8) & 0xFF
    }

    /// Returns the user mode interrupt level
    #[inline]
    pub fn uil(&self) -> usize {
        (self.bits >> 0) & 0xFF
    }
}

read_csr_as!(Mintstatus, 0x346);
