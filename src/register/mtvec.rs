//! mtvec register

/// mtvec register
#[derive(Clone, Copy, Debug)]
pub struct Mtvec {
    bits: usize,
}

/// Trap mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
    #[cfg(feature="clic")]
    Clic = 3,
}

/// CLIC sub mode
/// Currently there is just one mode
#[cfg(feature="clic")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubMode {
    Default = 0,
} 

impl Mtvec {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    #[inline]
    #[cfg(not(feature="clic"))]
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector base-address in CLIC mode
    #[inline]
    #[cfg(feature="clic")]
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b111111)
    }

    /// Returns the trap-vector mode
    #[inline]
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            #[cfg(feature="clic")]
            3 => Some(TrapMode::Clic),
            _ => None,
        }
    }

    /// Returns the trap-vector mode
    #[inline]
    #[cfg(feature="clic")]
    pub fn sub_mode(&self) -> Option<SubMode> {
        let mode = (self.bits & 0b111100) >> 2;
        match mode {
            0 => Some(SubMode::Default),
            _ => None,
        }
    }
}

read_csr_as!(Mtvec, 0x305);

write_csr!(0x305);

/// Writes the CSR
#[inline]
#[cfg(not(feature="clic"))]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    let bits = addr + mode as usize;
    _write(bits);
}


/// Writes the CSR including CLIC sub mode
#[inline]
#[cfg(feature="clic")]
pub unsafe fn write(addr: usize, submode:SubMode, mode: TrapMode) {
    let bits = addr + ((submode as usize) << 2) + mode as usize;
    _write(bits);
}