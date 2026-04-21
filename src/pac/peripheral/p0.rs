use super::super::P0_BASE;
use super::{
    P0Detectmode, P0Dir, P0Dirclr, P0Dirset, P0In, P0Latch, P0Out, P0Outclr, P0Outset, P0PinCnf,
};

#[repr(C)]
pub struct RegisterBlock {
    pub _reserved0: [u8; 1284],
    pub out: P0Out,
    pub outset: P0Outset,
    pub outclr: P0Outclr,
    pub in_: P0In,
    pub dir: P0Dir,
    pub dirset: P0Dirset,
    pub dirclr: P0Dirclr,
    pub latch: P0Latch,
    pub detectmode: P0Detectmode,
    pub _reserved1: [u8; 472],
    pub pin_cnf: [P0PinCnf; 32],
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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DetectmodeDetectmode {
    Default = 0,
    Ldetect = 1,
}
impl P0DetectmodeDetectmode {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Default),
            1 => Some(Self::Ldetect),
            _ => None,
        }
    }
}
impl From<P0DetectmodeDetectmode> for u8 {
    #[inline(always)]
    fn from(v: P0DetectmodeDetectmode) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DetectmodeDetectmode {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DirPin {
    Input = 0,
    Output = 1,
}
impl P0DirPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Input),
            1 => Some(Self::Output),
            _ => None,
        }
    }
}
impl From<P0DirPin> for u8 {
    #[inline(always)]
    fn from(v: P0DirPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DirPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DirclrPin {
    Input = 0,
    Output = 1,
}
impl P0DirclrPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Input),
            1 => Some(Self::Output),
            _ => None,
        }
    }
}
impl From<P0DirclrPin> for u8 {
    #[inline(always)]
    fn from(v: P0DirclrPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DirclrPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DirclrPin_1 {
    Clear = 1,
}
impl P0DirclrPin_1 {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Clear),
            _ => None,
        }
    }
}
impl From<P0DirclrPin_1> for u8 {
    #[inline(always)]
    fn from(v: P0DirclrPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DirclrPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DirsetPin {
    Input = 0,
    Output = 1,
}
impl P0DirsetPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Input),
            1 => Some(Self::Output),
            _ => None,
        }
    }
}
impl From<P0DirsetPin> for u8 {
    #[inline(always)]
    fn from(v: P0DirsetPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DirsetPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0DirsetPin_1 {
    Set = 1,
}
impl P0DirsetPin_1 {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Set),
            _ => None,
        }
    }
}
impl From<P0DirsetPin_1> for u8 {
    #[inline(always)]
    fn from(v: P0DirsetPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0DirsetPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0InPin {
    Low = 0,
    High = 1,
}
impl P0InPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Low),
            1 => Some(Self::High),
            _ => None,
        }
    }
}
impl From<P0InPin> for u8 {
    #[inline(always)]
    fn from(v: P0InPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0InPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0LatchPin {
    Notlatched = 0,
    Latched = 1,
}
impl P0LatchPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Notlatched),
            1 => Some(Self::Latched),
            _ => None,
        }
    }
}
impl From<P0LatchPin> for u8 {
    #[inline(always)]
    fn from(v: P0LatchPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0LatchPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0OutPin {
    Low = 0,
    High = 1,
}
impl P0OutPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Low),
            1 => Some(Self::High),
            _ => None,
        }
    }
}
impl From<P0OutPin> for u8 {
    #[inline(always)]
    fn from(v: P0OutPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0OutPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0OutclrPin {
    Low = 0,
    High = 1,
}
impl P0OutclrPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Low),
            1 => Some(Self::High),
            _ => None,
        }
    }
}
impl From<P0OutclrPin> for u8 {
    #[inline(always)]
    fn from(v: P0OutclrPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0OutclrPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0OutclrPin_1 {
    Clear = 1,
}
impl P0OutclrPin_1 {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Clear),
            _ => None,
        }
    }
}
impl From<P0OutclrPin_1> for u8 {
    #[inline(always)]
    fn from(v: P0OutclrPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0OutclrPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0OutsetPin {
    Low = 0,
    High = 1,
}
impl P0OutsetPin {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Low),
            1 => Some(Self::High),
            _ => None,
        }
    }
}
impl From<P0OutsetPin> for u8 {
    #[inline(always)]
    fn from(v: P0OutsetPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0OutsetPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0OutsetPin_1 {
    Set = 1,
}
impl P0OutsetPin_1 {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Set),
            _ => None,
        }
    }
}
impl From<P0OutsetPin_1> for u8 {
    #[inline(always)]
    fn from(v: P0OutsetPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0OutsetPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0PinCnfDir {
    Input = 0,
    Output = 1,
}
impl P0PinCnfDir {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Input),
            1 => Some(Self::Output),
            _ => None,
        }
    }
}
impl From<P0PinCnfDir> for u8 {
    #[inline(always)]
    fn from(v: P0PinCnfDir) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0PinCnfDir {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0PinCnfInput {
    Connect = 0,
    Disconnect = 1,
}
impl P0PinCnfInput {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Connect),
            1 => Some(Self::Disconnect),
            _ => None,
        }
    }
}
impl From<P0PinCnfInput> for u8 {
    #[inline(always)]
    fn from(v: P0PinCnfInput) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0PinCnfInput {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0PinCnfPull {
    Disabled = 0,
    Pulldown = 1,
    Pullup = 3,
}
impl P0PinCnfPull {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Pulldown),
            3 => Some(Self::Pullup),
            _ => None,
        }
    }
}
impl From<P0PinCnfPull> for u8 {
    #[inline(always)]
    fn from(v: P0PinCnfPull) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0PinCnfPull {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0PinCnfDrive {
    S0s1 = 0,
    H0s1 = 1,
    S0h1 = 2,
    H0h1 = 3,
    D0s1 = 4,
    D0h1 = 5,
    S0d1 = 6,
    H0d1 = 7,
}
impl P0PinCnfDrive {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::S0s1),
            1 => Some(Self::H0s1),
            2 => Some(Self::S0h1),
            3 => Some(Self::H0h1),
            4 => Some(Self::D0s1),
            5 => Some(Self::D0h1),
            6 => Some(Self::S0d1),
            7 => Some(Self::H0d1),
            _ => None,
        }
    }
}
impl From<P0PinCnfDrive> for u8 {
    #[inline(always)]
    fn from(v: P0PinCnfDrive) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0PinCnfDrive {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P0PinCnfSense {
    Disabled = 0,
    High = 2,
    Low = 3,
}
impl P0PinCnfSense {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            2 => Some(Self::High),
            3 => Some(Self::Low),
            _ => None,
        }
    }
}
impl From<P0PinCnfSense> for u8 {
    #[inline(always)]
    fn from(v: P0PinCnfSense) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P0PinCnfSense {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}
