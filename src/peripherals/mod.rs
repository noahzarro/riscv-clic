pub mod clic;
use core::ops;


pub struct Peripherals {
    pub CLIC: CLIC
}

/// Nested Vector Interrupt Controller
#[allow(clippy::upper_case_acronyms)]
pub struct CLIC {
    //_marker: PhantomData<*const ()>,
}

unsafe impl Send for CLIC {}

impl CLIC {
    /// Pointer to the register block
    pub const PTR: *const clic::RegisterBlock = 0xE000_E100 as *const _;
}

impl ops::Deref for CLIC {
    type Target = self::clic::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}