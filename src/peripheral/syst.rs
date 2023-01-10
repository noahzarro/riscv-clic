//! SysTick: System Timer

use volatile_register::{RW,WO};

use crate::peripheral::SYST;

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

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Low register config
    pub cfg_low: RW<u32>,
    /// High register config
    pub cfg_high: RW<u32>,
    /// Low register count
    pub cnt_low: RW<u32>,
    /// High register count
    pub cnt_high: RW<u32>,
    /// Low register compare
    pub cmp_low: RW<u32>,
    /// High register compare
    pub cmp_high: RW<u32>,
    /// Low command register start
    pub start_low: WO<u32>,
    /// High command register start
    pub start_high: WO<u32>,
    /// Low command register reset
    pub reset_low: WO<u32>,
    /// High command register reset
    pub reset_high: WO<u32>,
}

impl SYST {
    #[inline]
    pub fn enable_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 0, 0, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 0, 0, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn reset_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 1, 1, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_interrupt_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 2, 2, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_interrupt_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 2, 2, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_event_mask_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 3, 3, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_event_mask_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 3, 3, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn set_continuos_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 4, 4, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn set_cycle_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 4, 4, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_one_shot_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 5, 5, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_one_shot_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 5, 5, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_pre_scaler_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 6, 6, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_pre_scaler_mode_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 6, 6, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn set_ffl_clk_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 7, 7, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn set_reference_clk_lo() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 7, 7, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn set_pre_scale_value_lo(value:u32) {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 15, 8, value);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_cascaded_mode() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 31, 31, 1);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn disable_cascaded_mode() {
        unsafe { 
            let before = (*Self::PTR).cfg_low.read();
            let changed = write_bits(before, 31, 31, 0);
            (*Self::PTR).cfg_low.write(changed)
        }
    }

    #[inline]
    pub fn enable_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 0, 0, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn disable_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 0, 0, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn reset_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 1, 1, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn enable_interrupt_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 2, 2, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn disable_interrupt_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 2, 2, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn enable_event_mask_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 3, 3, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn disable_event_mask_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 3, 3, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn set_continuos_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 4, 4, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn set_cycle_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 4, 4, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn enable_one_shot_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 5, 5, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn disable_one_shot_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 5, 5, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn enable_pre_scaler_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 6, 6, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn disable_pre_scaler_mode_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 6, 6, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn set_ffl_clk_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 7, 7, 0);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn set_reference_clk_hi() {
        unsafe { 
            let before = (*Self::PTR).cfg_high.read();
            let changed = write_bits(before, 7, 7, 1);
            (*Self::PTR).cfg_high.write(changed)
        }
    }

    #[inline]
    pub fn set_counter_lo(value:u32) {
        unsafe { 
            (*Self::PTR).cnt_low.write(value)
        }
    }

    #[inline]
    pub fn get_counter_lo() -> u32 {
        unsafe { 
            (*Self::PTR).cnt_low.read()
        }
    }


    #[inline]
    pub fn set_compare_lo(value:u32) {
        unsafe { 
            (*Self::PTR).cmp_low.write(value)
        }
    }

    #[inline]
    pub fn get_compare_lo() -> u32 {
        unsafe { 
            (*Self::PTR).cmp_low.read()
        }
    }

    #[inline]
    pub fn start_command_lo() {
        unsafe { 
            (*Self::PTR).cmp_low.write(1)
        }
    }

    #[inline]
    pub fn reset_command_lo() {
        unsafe { 
            (*Self::PTR).cmp_low.write(1)
        }
    }

    #[inline]
    pub fn set_counter_hi(value:u32) {
        unsafe { 
            (*Self::PTR).cnt_high.write(value)
        }
    }

    #[inline]
    pub fn get_counter_hi() -> u32 {
        unsafe { 
            (*Self::PTR).cnt_high.read()
        }
    }


    #[inline]
    pub fn set_compare_hi(value:u32) {
        unsafe { 
            (*Self::PTR).cmp_high.write(value)
        }
    }

    #[inline]
    pub fn get_compare_hi() -> u32 {
        unsafe { 
            (*Self::PTR).cmp_high.read()
        }
    }

    #[inline]
    pub fn start_command_hi() {
        unsafe { 
            (*Self::PTR).cmp_high.write(1)
        }
    }

    #[inline]
    pub fn reset_command_hi() {
        unsafe { 
            (*Self::PTR).cmp_high.write(1)
        }
    }

}
