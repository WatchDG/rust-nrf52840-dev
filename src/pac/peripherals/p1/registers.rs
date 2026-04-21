use crate::pac::peripherals::{RO, RW, W1C, W1S};
use crate::{impl_ro_register, impl_rw_register, impl_w1c_register, impl_w1s_register};

#[repr(transparent)]
pub struct P1Out(RW<u32>);
impl_rw_register!(P1Out);

#[repr(transparent)]
pub struct P1Outset(W1S<u32>);
impl_w1s_register!(P1Outset);

#[repr(transparent)]
pub struct P1Outclr(W1C<u32>);
impl_w1c_register!(P1Outclr);

#[repr(transparent)]
pub struct P1In(RO<u32>);
impl_ro_register!(P1In);

#[repr(transparent)]
pub struct P1Dir(RW<u32>);
impl_rw_register!(P1Dir);

#[repr(transparent)]
pub struct P1Dirset(W1S<u32>);
impl_w1s_register!(P1Dirset);

#[repr(transparent)]
pub struct P1Dirclr(W1C<u32>);
impl_w1c_register!(P1Dirclr);

#[repr(transparent)]
pub struct P1Latch(RW<u32>);
impl_rw_register!(P1Latch);

#[repr(transparent)]
pub struct P1Detectmode(RW<u32>);
impl_rw_register!(P1Detectmode);

#[repr(transparent)]
pub struct P1PinCnf(RW<u32>);
impl_rw_register!(P1PinCnf);
