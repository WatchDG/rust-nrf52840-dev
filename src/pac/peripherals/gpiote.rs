use core::convert::TryFrom;

use super::super::{GPIOTE_BASE, RW, WO};

#[repr(transparent)]
pub struct TasksOut(WO<u32>);
impl TasksOut {
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct TasksSet(WO<u32>);
impl TasksSet {
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct TasksClr(WO<u32>);
impl TasksClr {
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(transparent)]
pub struct EventsIn(RW<u32>);
impl EventsIn {
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
pub struct EventsPort(RW<u32>);
impl EventsPort {
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
pub struct IntenSet(RW<u32>);
impl IntenSet {
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
pub struct IntenClr(RW<u32>);
impl IntenClr {
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
pub struct Config(RW<u32>);
impl Config {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.0.read()
    }
    #[inline(always)]
    pub fn write(&self, v: u32) {
        self.0.write(v)
    }
}

#[repr(C)]
pub struct RegisterBlock {
    pub tasks_out: [TasksOut; 8],
    pub _reserved0: [u8; 16],
    pub tasks_set: [TasksSet; 8],
    pub _reserved1: [u8; 16],
    pub tasks_clr: [TasksClr; 8],
    pub _reserved2: [u8; 128],
    pub events_in: [EventsIn; 8],
    pub _reserved3: [u8; 92],
    pub events_port: EventsPort,
    pub _reserved4: [u8; 388],
    pub intenset: IntenSet,
    pub intenclr: IntenClr,
    pub _reserved5: [u8; 516],
    pub config: [Config; 8],
}
impl RegisterBlock {
    #[inline(always)]
    pub fn reset(&self) {
        for r in self.tasks_out.iter() {
            r.write(0x00000000u32);
        }
        for r in self.tasks_set.iter() {
            r.write(0x00000000u32);
        }
        for r in self.tasks_clr.iter() {
            r.write(0x00000000u32);
        }
        for r in self.events_in.iter() {
            r.write(0x00000000u32);
        }
        self.events_port.write(0x00000000u32);
        self.intenset.write(0x00000000u32);
        self.intenclr.write(0x00000000u32);
        for r in self.config.iter() {
            r.write(0x00000000u32);
        }
    }
}
pub const PTR: *const RegisterBlock = GPIOTE_BASE as *const RegisterBlock;
pub const PTR_MUT: *mut RegisterBlock = GPIOTE_BASE as *mut RegisterBlock;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConfigMode {
    Disabled = 0,
    Event = 1,
    Task = 3,
}
impl ConfigMode {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Event),
            3 => Some(Self::Task),
            _ => None,
        }
    }
}
impl From<ConfigMode> for u8 {
    #[inline(always)]
    fn from(v: ConfigMode) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for ConfigMode {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConfigPolarity {
    None = 0,
    LoToHi = 1,
    HiToLo = 2,
    Toggle = 3,
}
impl ConfigPolarity {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::LoToHi),
            2 => Some(Self::HiToLo),
            3 => Some(Self::Toggle),
            _ => None,
        }
    }
}
impl From<ConfigPolarity> for u8 {
    #[inline(always)]
    fn from(v: ConfigPolarity) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for ConfigPolarity {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConfigOutInit {
    Low = 0,
    High = 1,
}
impl ConfigOutInit {
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
impl From<ConfigOutInit> for u8 {
    #[inline(always)]
    fn from(v: ConfigOutInit) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for ConfigOutInit {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventsInEventsIn {
    NotGenerated = 0,
    Generated = 1,
}
impl EventsInEventsIn {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::NotGenerated),
            1 => Some(Self::Generated),
            _ => None,
        }
    }
}
impl From<EventsInEventsIn> for u8 {
    #[inline(always)]
    fn from(v: EventsInEventsIn) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for EventsInEventsIn {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventsPortEventsPort {
    NotGenerated = 0,
    Generated = 1,
}
impl EventsPortEventsPort {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::NotGenerated),
            1 => Some(Self::Generated),
            _ => None,
        }
    }
}
impl From<EventsPortEventsPort> for u8 {
    #[inline(always)]
    fn from(v: EventsPortEventsPort) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for EventsPortEventsPort {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntenclrIn {
    Disabled = 0,
    Enabled = 1,
}
impl GpioTEIntenclrIn {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }
}
impl From<GpioTEIntenclrIn> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntenclrIn) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntenclrIn {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntenclrIn_1 {
    Clear = 1,
}
impl GpioTEIntenclrIn_1 {
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
impl From<GpioTEIntenclrIn_1> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntenclrIn_1) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntenclrIn_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntenclrPort {
    Disabled = 0,
    Enabled = 1,
}
impl GpioTEIntenclrPort {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }
}
impl From<GpioTEIntenclrPort> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntenclrPort) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntenclrPort {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntenclrPort_1 {
    Clear = 1,
}
impl GpioTEIntenclrPort_1 {
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
impl From<GpioTEIntenclrPort_1> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntenclrPort_1) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntenclrPort_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntensetIn {
    Disabled = 0,
    Enabled = 1,
}
impl GpioTEIntensetIn {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }
}
impl From<GpioTEIntensetIn> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntensetIn) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntensetIn {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntensetIn_1 {
    Set = 1,
}
impl GpioTEIntensetIn_1 {
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
impl From<GpioTEIntensetIn_1> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntensetIn_1) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntensetIn_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntensetPort {
    Disabled = 0,
    Enabled = 1,
}
impl GpioTEIntensetPort {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }
}
impl From<GpioTEIntensetPort> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntensetPort) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntensetPort {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTEIntensetPort_1 {
    Set = 1,
}
impl GpioTEIntensetPort_1 {
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
impl From<GpioTEIntensetPort_1> for u8 {
    #[inline(always)]
    fn from(v: GpioTEIntensetPort_1) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTEIntensetPort_1 {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTETasksClrTasksClr {
    Trigger = 1,
}
impl GpioTETasksClrTasksClr {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Trigger),
            _ => None,
        }
    }
}
impl From<GpioTETasksClrTasksClr> for u8 {
    #[inline(always)]
    fn from(v: GpioTETasksClrTasksClr) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTETasksClrTasksClr {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTETasksOutTasksOut {
    Trigger = 1,
}
impl GpioTETasksOutTasksOut {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Trigger),
            _ => None,
        }
    }
}
impl From<GpioTETasksOutTasksOut> for u8 {
    #[inline(always)]
    fn from(v: GpioTETasksOutTasksOut) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTETasksOutTasksOut {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioTETasksSetTasksSet {
    Trigger = 1,
}
impl GpioTETasksSetTasksSet {
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    pub const fn from_bits(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Trigger),
            _ => None,
        }
    }
}
impl From<GpioTETasksSetTasksSet> for u8 {
    #[inline(always)]
    fn from(v: GpioTETasksSetTasksSet) -> u8 {
        v.bits()
    }
}
impl TryFrom<u8> for GpioTETasksSetTasksSet {
    type Error = ();
    #[inline(always)]
    fn try_from(v: u8) -> core::result::Result<Self, ()> {
        Self::from_bits(v).ok_or(())
    }
}
