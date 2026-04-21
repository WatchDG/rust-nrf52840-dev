use super::super::P1_BASE;
use super::{
    P1Detectmode, P1Dir, P1Dirclr, P1Dirset, P1In, P1Latch, P1Out, P1Outclr, P1Outset, P1PinCnf,
};

#[repr(C)]
pub struct RegisterBlock {
    pub _reserved0: [u8; 1284],
    pub out: P1Out,
    pub outset: P1Outset,
    pub outclr: P1Outclr,
    pub in_: P1In,
    pub dir: P1Dir,
    pub dirset: P1Dirset,
    pub dirclr: P1Dirclr,
    pub latch: P1Latch,
    pub detectmode: P1Detectmode,
    pub _reserved1: [u8; 472],
    pub pin_cnf: [P1PinCnf; 32],
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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DetectmodeDetectmode {
    Default = 0,
    Ldetect = 1,
}
impl P1DetectmodeDetectmode {
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
impl From<P1DetectmodeDetectmode> for u8 {
    #[inline(always)]
    fn from(v: P1DetectmodeDetectmode) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DetectmodeDetectmode {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DirPin {
    Input = 0,
    Output = 1,
}
impl P1DirPin {
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
impl From<P1DirPin> for u8 {
    #[inline(always)]
    fn from(v: P1DirPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DirPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DirclrPin {
    Input = 0,
    Output = 1,
}
impl P1DirclrPin {
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
impl From<P1DirclrPin> for u8 {
    #[inline(always)]
    fn from(v: P1DirclrPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DirclrPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DirclrPin_1 {
    Clear = 1,
}
impl P1DirclrPin_1 {
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
impl From<P1DirclrPin_1> for u8 {
    #[inline(always)]
    fn from(v: P1DirclrPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DirclrPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DirsetPin {
    Input = 0,
    Output = 1,
}
impl P1DirsetPin {
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
impl From<P1DirsetPin> for u8 {
    #[inline(always)]
    fn from(v: P1DirsetPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DirsetPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1DirsetPin_1 {
    Set = 1,
}
impl P1DirsetPin_1 {
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
impl From<P1DirsetPin_1> for u8 {
    #[inline(always)]
    fn from(v: P1DirsetPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1DirsetPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1InPin {
    Low = 0,
    High = 1,
}
impl P1InPin {
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
impl From<P1InPin> for u8 {
    #[inline(always)]
    fn from(v: P1InPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1InPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1LatchPin {
    Notlatched = 0,
    Latched = 1,
}
impl P1LatchPin {
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
impl From<P1LatchPin> for u8 {
    #[inline(always)]
    fn from(v: P1LatchPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1LatchPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1OutPin {
    Low = 0,
    High = 1,
}
impl P1OutPin {
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
impl From<P1OutPin> for u8 {
    #[inline(always)]
    fn from(v: P1OutPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1OutPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1OutclrPin {
    Low = 0,
    High = 1,
}
impl P1OutclrPin {
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
impl From<P1OutclrPin> for u8 {
    #[inline(always)]
    fn from(v: P1OutclrPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1OutclrPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1OutclrPin_1 {
    Clear = 1,
}
impl P1OutclrPin_1 {
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
impl From<P1OutclrPin_1> for u8 {
    #[inline(always)]
    fn from(v: P1OutclrPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1OutclrPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1OutsetPin {
    Low = 0,
    High = 1,
}
impl P1OutsetPin {
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
impl From<P1OutsetPin> for u8 {
    #[inline(always)]
    fn from(v: P1OutsetPin) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1OutsetPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1OutsetPin_1 {
    Set = 1,
}
impl P1OutsetPin_1 {
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
impl From<P1OutsetPin_1> for u8 {
    #[inline(always)]
    fn from(v: P1OutsetPin_1) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1OutsetPin_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1PinCnfDir {
    Input = 0,
    Output = 1,
}
impl P1PinCnfDir {
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
impl From<P1PinCnfDir> for u8 {
    #[inline(always)]
    fn from(v: P1PinCnfDir) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1PinCnfDir {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1PinCnfInput {
    Connect = 0,
    Disconnect = 1,
}
impl P1PinCnfInput {
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
impl From<P1PinCnfInput> for u8 {
    #[inline(always)]
    fn from(v: P1PinCnfInput) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1PinCnfInput {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1PinCnfPull {
    Disabled = 0,
    Pulldown = 1,
    Pullup = 3,
}
impl P1PinCnfPull {
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
impl From<P1PinCnfPull> for u8 {
    #[inline(always)]
    fn from(v: P1PinCnfPull) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1PinCnfPull {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1PinCnfDrive {
    S0s1 = 0,
    H0s1 = 1,
    S0h1 = 2,
    H0h1 = 3,
    D0s1 = 4,
    D0h1 = 5,
    S0d1 = 6,
    H0d1 = 7,
}
impl P1PinCnfDrive {
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
impl From<P1PinCnfDrive> for u8 {
    #[inline(always)]
    fn from(v: P1PinCnfDrive) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1PinCnfDrive {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum P1PinCnfSense {
    Disabled = 0,
    High = 2,
    Low = 3,
}
impl P1PinCnfSense {
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
impl From<P1PinCnfSense> for u8 {
    #[inline(always)]
    fn from(v: P1PinCnfSense) -> u8 {
        v.bits()
    }
}
impl core::convert::TryFrom<u8> for P1PinCnfSense {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}
