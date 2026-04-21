pub mod enums;
pub mod registers;

use crate::pac::P1_BASE;
// use crate::pac::peripherals::{
//     P1Detectmode, P1Dir, P1Dirclr, P1Dirset, P1In, P1Latch, P1Out, P1Outclr, P1Outset, P1PinCnf,
// };

#[repr(C)]
pub struct RegisterBlock {
    pub _reserved0: [u8; 1284],
    pub out: registers::P1Out,
    pub outset: registers::P1Outset,
    pub outclr: registers::P1Outclr,
    pub in_: registers::P1In,
    pub dir: registers::P1Dir,
    pub dirset: registers::P1Dirset,
    pub dirclr: registers::P1Dirclr,
    pub latch: registers::P1Latch,
    pub detectmode: registers::P1Detectmode,
    pub _reserved1: [u8; 472],
    pub pin_cnf: [registers::P1PinCnf; 32],
}

impl RegisterBlock {
    #[inline(always)]
    pub fn reset(&self) {
        self.out.write(0x00000000u32);
        self.dir.write(0x00000000u32);
        self.latch.write(0x00000000u32);
        self.detectmode.write(0x00000000u32);
        for r in self.pin_cnf.iter() {
            r.write(0x00000002u32);
        }
    }
}

pub const PTR: *const RegisterBlock = P1_BASE as *const RegisterBlock;
pub const PTR_MUT: *mut RegisterBlock = P1_BASE as *mut RegisterBlock;
