use crate::pac::peripherals::{RO, RW, W1C, W1S};
use crate::{impl_ro_register, impl_rw_register, impl_w1c_register, impl_w1s_register};

#[repr(transparent)]
pub struct P0Out(RW<u32>);
impl_rw_register!(P0Out);

#[repr(transparent)]
pub struct P0OutSet(W1S<u32>);
impl_w1s_register!(P0OutSet);

#[repr(transparent)]
pub struct P0OutClr(W1C<u32>);
impl_w1c_register!(P0OutClr);

#[repr(transparent)]
pub struct P0In(RO<u32>);
impl_ro_register!(P0In);

#[repr(transparent)]
pub struct P0Dir(RW<u32>);
impl_rw_register!(P0Dir);

#[repr(transparent)]
pub struct P0DirSet(W1S<u32>);
impl_w1s_register!(P0DirSet);

#[repr(transparent)]
pub struct P0DirClr(W1C<u32>);
impl_w1c_register!(P0DirClr);

#[repr(transparent)]
pub struct P0Latch(RW<u32>);
impl_rw_register!(P0Latch);

#[repr(transparent)]
pub struct P0DetectMode(RW<u32>);
impl_rw_register!(P0DetectMode);

#[repr(transparent)]
pub struct P0PinCnf(RW<u32>);
impl_rw_register!(P0PinCnf);
