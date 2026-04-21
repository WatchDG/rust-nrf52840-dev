use crate::enum_bits;

enum_bits! {
    DetectMode: u8,
    Default = 0,
    LDetect = 1,
}

enum_bits! {
    P0DirPin: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P0InPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P0LatchPin: u8,
    Notlatched = 0,
    Latched = 1,
}

enum_bits! {
    P0OutPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P0OutClrPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P0OutclrPin_1: u8,
    Clear = 1,
}

enum_bits! {
    P0OutsetPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P0OutsetPin_1: u8,
    Set = 1,
}

enum_bits! {
    P0PinCnfDir: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P0PinCnfInput: u8,
    Connect = 0,
    Disconnect = 1,
}

enum_bits! {
    P0PinCnfPull: u8,
    Disabled = 0,
    PullDown = 1,
    PullUp = 3,
}

enum_bits! {
    P0PinCnfDrive: u8,
    S0s1 = 0,
    H0s1 = 1,
    S0h1 = 2,
    H0h1 = 3,
    D0s1 = 4,
    D0h1 = 5,
    S0d1 = 6,
    H0d1 = 7,
}

enum_bits! {
    P0PinCnfSense: u8,
    Disabled = 0,
    High = 2,
    Low = 3,
}
