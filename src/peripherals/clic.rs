#[cfg(not(armv6m))]
use volatile_register::RO;
use volatile_register::RW;

use crate::interrupt::InterruptNumber;
use crate::peripherals::CLIC;
use crate::register::mcause;

/// Interrupt block
#[repr(C)]
pub struct InterruptBlock {
    pub ip: RW<u32>,
    pub ie: RW<u32>,
    pub attr: RW<u32>,
    pub ctl: RW<u32>,
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    pub cliccfg: RW<u32>,
    pub clicinfo: RO<u32>,
    reserved: [u32; 0x1000 - 4 * 2],
    pub intcfg: [InterruptBlock; 4096],
}

impl CLIC {
    /// Disables `interrupt`
    #[inline]
    pub fn mask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ie.write(0) }
    }

    /// Enables `interrupt`
    ///
    /// This function is `unsafe` because it can break mask-based critical sections
    #[inline]
    pub fn unmask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ie.write(1) }
    }

    /// Returns the CLIC priority of `interrupt`
    #[inline]
    pub fn get_priority<I>(interrupt: I) -> u8
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::PTR).intcfg[nr].ctl.read() as u8 }
    }

    /// Is `interrupt` active or pre-empted and stacked
    #[cfg(not(armv6m))]
    #[inline]
    pub fn is_active<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();

        let mask = 0x0000_0FFF;

        // NOTE(unsafe) atomic read with no side effects
        (mcause::read().bits() & mask) == nr
    }

    /// Checks if `interrupt` is enabled
    #[inline]
    pub fn is_enabled<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ie.read() == 1 }

    }

    /// Checks if `interrupt` is pending
    #[inline]
    pub fn is_pending<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ip.read() == 1 }

    }

    /// Forces `interrupt` into pending state
    #[inline]
    pub fn pend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ip.write(1) }
    }

    /// Sets the "priority" of `interrupt` to `prio`
    #[inline]
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        (*Self::PTR).intcfg[nr].ctl.write(prio.into())
    }

    /// Clears `interrupt`'s pending state
    #[inline]
    pub fn unpend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ip.write(0) }
    }

}
