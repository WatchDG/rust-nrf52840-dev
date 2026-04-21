pub mod enums;
pub mod registers;

use crate::pac::P0_BASE;

#[repr(C)]
pub struct RegisterBlock {
    pub _reserved0: [u8; 1284],
    pub out: registers::P0Out,
    pub out_set: registers::P0OutSet,
    pub out_clr: registers::P0OutClr,
    pub r#in: registers::P0In,
    pub dir: registers::P0Dir,
    pub dir_set: registers::P0DirSet,
    pub dir_clr: registers::P0DirClr,
    pub latch: registers::P0Latch,
    pub detectmode: registers::P0DetectMode,
    pub _reserved1: [u8; 472],
    pub pin_cnf: [registers::P0PinCnf; 32],
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

pub const PTR: *const RegisterBlock = P0_BASE as *const RegisterBlock;
pub const PTR_MUT: *mut RegisterBlock = P0_BASE as *mut RegisterBlock;
