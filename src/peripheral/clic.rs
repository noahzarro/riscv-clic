#[cfg(not(armv6m))]
use volatile_register::RO;
use volatile_register::RW;

use crate::interrupt::InterruptNumber;
use crate::peripheral::CLIC;
use crate::register::mcause;

/// Writes the `bits` into `base_values` at pos [low_bit, high_bit] both included
#[inline]
fn write_bits(base_value: u32, high_bit: u8, low_bit: u8, bits: u32) -> u32 {
    let mut mask = 0;
    for i in low_bit..high_bit + 1 {
        mask += 1 << i;
    }
    let offset = low_bit;
    (base_value & !mask) | bits << offset
}

/// Reads the bits from `base_values` at pos [low_bit, high_bit] both included
#[inline]
fn read_bits(base_value: u32, high_bit: u8, low_bit: u8) -> u32 {
    let mut mask = 0;
    for i in low_bit..high_bit + 1 {
        mask += 1 << i;
    }
    let offset = low_bit;
    (base_value & mask) >> offset
}

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

/// Trigger enum
pub enum Trigger {
    LevelPositive = 0,
    EdgePositive = 1,
    LevelNegative = 2,
    EdgeNegative = 3,
}

impl CLIC {
    //* IE
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

    //* CTL
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

    /// Sets the "priority" of `interrupt` to `prio`
    #[inline]
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        (*Self::PTR).intcfg[nr].ctl.write(prio.into())
    }

    /// Is `interrupt` active or pre-empted and stacked
    /// TODO: Check this
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

    //* IP
    /// Forces `interrupt` into pending state
    #[inline]
    pub fn pend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).intcfg[nr].ip.write(1) }
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

    //* ATTR
    /// Enables "selective hardware vectoring" of `interrupt`
    #[inline]
    pub unsafe fn enable_shv<I>(&mut self, interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let before = (*Self::PTR).intcfg[nr].attr.read();
        let edited = write_bits(before, 0, 0, 1);

        (*Self::PTR).intcfg[nr].attr.write(edited)
    }

    /// Disables "selective hardware vectoring" of `interrupt`
    #[inline]
    pub unsafe fn disable_shv<I>(&mut self, interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let before = (*Self::PTR).intcfg[nr].attr.read();
        let edited = write_bits(before, 0, 0, 0);

        (*Self::PTR).intcfg[nr].attr.write(edited)
    }

    /// Sets "trigger" of `interrupt`
    #[inline]
    pub unsafe fn set_trig<I>(&mut self, interrupt: I, trig: Trigger)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        let before = (*Self::PTR).intcfg[nr].attr.read();
        let edited = write_bits(before, 2, 1, trig as u32);
        (*Self::PTR).intcfg[nr].attr.write(edited)
    }

    // TODO: implement attr mode bits if needed

    //* CLIC CFG

    // Sets number of bits used for mode int attr fields
    pub unsafe fn set_mode_bit_width(&mut self, nr_bits: u32) {
        let before = (*Self::PTR).cliccfg.read();
        let edited = write_bits(before, 6, 5, nr_bits);
        (*Self::PTR).cliccfg.write(edited)
    }

    // Gets number of bits used for mode int attr fields
    pub unsafe fn get_mode_bit_width(&mut self) -> u32 {
        let before = (*Self::PTR).cliccfg.read();
        read_bits(before, 6, 5)
    }

    // Sets number of bits used for interrupt level value
    pub unsafe fn set_level_bit_width(&mut self, nr_bits: u32) {
        let before = (*Self::PTR).cliccfg.read();
        let edited = write_bits(before, 4, 1, nr_bits);
        (*Self::PTR).cliccfg.write(edited)
    }

    // Gets number of bits used for interrupt level value
    pub unsafe fn get_level_bit_width(&mut self) -> u32 {
        let before = (*Self::PTR).cliccfg.read();
        read_bits(before, 4, 1)
    }

    // Gets flag if vectored interrupt handling is implemented in hardware
    pub unsafe fn has_interrupt_vectoring(&mut self) -> bool {
        let before = (*Self::PTR).cliccfg.read();
        read_bits(before, 0, 0) != 1
    }

    //* CLIC INFO
    // Gets actual number of maximum interrupt inputs supported in this implementation
    pub unsafe fn get_num_int(&mut self) -> u32 {
        let before = (*Self::PTR).clicinfo.read();
        read_bits(before, 30, 25)
    }

    // Gets how many hardware bits are actually implemented in the clicintctl registers
    pub fn get_possible_level_bits(&mut self) -> u32 {
        let before = unsafe { (*Self::PTR).clicinfo.read() };
        read_bits(before, 24, 21)
    }

    // Gets version
    pub unsafe fn get_version(&mut self) -> u32 {
        let before = (*Self::PTR).clicinfo.read();
        read_bits(before, 20, 13)
    }

    // Gets number of maximum interrupt inputs supported
    pub unsafe fn get_max_interrupts(&mut self) -> u32 {
        let before = (*Self::PTR).clicinfo.read();
        read_bits(before, 12, 0)
    }
}
