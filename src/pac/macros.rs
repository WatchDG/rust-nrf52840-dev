#[macro_export]
macro_rules! impl_ro_register {
    ($name:ident) => {
        impl $name {
            #[inline(always)]
            pub fn read(&self) -> u32 {
                self.0.read()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_wo_register {
    ($name:ident) => {
        impl $name {
            #[inline(always)]
            pub fn write(&self, v: u32) {
                self.0.write(v)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_rw_register {
    ($name:ident) => {
        impl $name {
            #[inline(always)]
            pub fn read(&self) -> u32 {
                self.0.read()
            }
            #[inline(always)]
            pub fn write(&self, v: u32) {
                self.0.write(v)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_w1s_register {
    ($name:ident) => {
        impl $name {
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
    };
}

#[macro_export]
macro_rules! impl_w1c_register {
    ($name:ident) => {
        impl $name {
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
    };
}

#[macro_export]
macro_rules! enum_bits {
    ($name:ident : $type:ty , $($variant:ident = $value:expr),* $(,)?) => {
        #[repr($type)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $name {
            $($variant = $value),*
        }
        impl $name {
            #[inline(always)]
            pub const fn bits(self) -> $type {
                self as $type
            }
            #[inline(always)]
            pub const fn from_bits(v: $type) -> Option<Self> {
                match v {
                    $($value => Some(Self::$variant)),*,
                    _ => None,
                }
            }
        }
        impl From<$name> for $type {
            #[inline(always)]
            fn from(v: $name) -> $type {
                v.bits()
            }
        }
        impl core::convert::TryFrom<$type> for $name {
            type Error = ();
            #[inline(always)]
            fn try_from(v: $type) -> core::result::Result<Self, ()> {
                Self::from_bits(v).ok_or(())
            }
        }
    };
}
