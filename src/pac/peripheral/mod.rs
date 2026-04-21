use super::{RO, RW, W1C, W1S};

pub mod gpiote;
pub mod p0;
pub mod p1;

#[repr(transparent)]
pub struct P0Out(RW<u32>);
impl P0Out {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P0Outset(W1S<u32>);
impl P0Outset {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn set_bits(&self, mask: u32) {
        self.0.set_bits(mask)
    }
}

#[repr(transparent)]
pub struct P0Outclr(W1C<u32>);
impl P0Outclr {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn clear_bits(&self, mask: u32) {
        self.0.clear_bits(mask)
    }
}

#[repr(transparent)]
pub struct P0In(RO<u32>);
impl P0In {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
}

#[repr(transparent)]
pub struct P0Dir(RW<u32>);
impl P0Dir {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P0Dirset(W1S<u32>);
impl P0Dirset {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn set_bits(&self, mask: u32) {
        self.0.set_bits(mask)
    }
}

#[repr(transparent)]
pub struct P0Dirclr(W1C<u32>);
impl P0Dirclr {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn clear_bits(&self, mask: u32) {
        self.0.clear_bits(mask)
    }
}

#[repr(transparent)]
pub struct P0Latch(RW<u32>);
impl P0Latch {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P0Detectmode(RW<u32>);
impl P0Detectmode {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P0PinCnf(RW<u32>);
impl P0PinCnf {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P1Out(RW<u32>);
impl P1Out {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P1Outset(W1S<u32>);
impl P1Outset {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn set_bits(&self, mask: u32) {
        self.0.set_bits(mask)
    }
}

#[repr(transparent)]
pub struct P1Outclr(W1C<u32>);
impl P1Outclr {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn clear_bits(&self, mask: u32) {
        self.0.clear_bits(mask)
    }
}

#[repr(transparent)]
pub struct P1In(RO<u32>);
impl P1In {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
}

#[repr(transparent)]
pub struct P1Dir(RW<u32>);
impl P1Dir {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P1Dirset(W1S<u32>);
impl P1Dirset {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn set_bits(&self, mask: u32) {
        self.0.set_bits(mask)
    }
}

#[repr(transparent)]
pub struct P1Dirclr(W1C<u32>);
impl P1Dirclr {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
    #[inline(always)]
    pub fn clear_bits(&self, mask: u32) {
        self.0.clear_bits(mask)
    }
}

#[repr(transparent)]
pub struct P1Latch(RW<u32>);
impl P1Latch {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P1Detectmode(RW<u32>);
impl P1Detectmode {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct P1PinCnf(RW<u32>);
impl P1PinCnf {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}
