use core::convert::TryFrom;
use core::result::Result;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DetectMode {
    Default = 0,
    LDetect = 1,
}
impl DetectMode {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Default),
            1 => Some(Self::LDetect),
            _ => None,
        }
    }
}
impl From<DetectMode> for u8 {
    #[inline(always)]
    fn from(v: DetectMode) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for DetectMode {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> Result<Self, ()> {
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
impl TryFrom<u8> for P0DirPin {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}
