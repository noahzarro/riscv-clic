pub mod clic;
use core::{ops, marker::PhantomData};


pub struct Peripherals {
    pub CLIC: CLIC,
    _priv: (),
}

// NOTE `no_mangle` is used here to prevent linking different minor versions of this crate as that
// would let you `take` the core peripherals more than once (one per minor version)
#[no_mangle]
static CORE_PERIPHERALS: () = ();

/// Set to `true` when `take` or `steal` was called to make `Peripherals` a singleton.
static mut TAKEN: bool = false;

impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { TAKEN } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }

    /// Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        TAKEN = true;

        Peripherals {
            CLIC: CLIC {
                _marker: PhantomData,
            },
            _priv: (),
        }
    }
}

/// Nested Vector Interrupt Controller
#[allow(clippy::upper_case_acronyms)]
pub struct CLIC {
    _marker: PhantomData<*const ()>,
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
