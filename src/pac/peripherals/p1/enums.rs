use crate::enum_bits;

enum_bits! {
    P1DetectmodeDetectmode: u8,
    Default = 0,
    Ldetect = 1,
}

enum_bits! {
    P1DirPin: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P1DirclrPin: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P1DirclrPin_1: u8,
    Clear = 1,
}

enum_bits! {
    P1DirsetPin: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P1DirsetPin_1: u8,
    Set = 1,
}

enum_bits! {
    P1InPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P1LatchPin: u8,
    Notlatched = 0,
    Latched = 1,
}

enum_bits! {
    P1OutPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P1OutclrPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P1OutclrPin_1: u8,
    Clear = 1,
}

enum_bits! {
    P1OutsetPin: u8,
    Low = 0,
    High = 1,
}

enum_bits! {
    P1OutsetPin_1: u8,
    Set = 1,
}

enum_bits! {
    P1PinCnfDir: u8,
    Input = 0,
    Output = 1,
}

enum_bits! {
    P1PinCnfInput: u8,
    Connect = 0,
    Disconnect = 1,
}

enum_bits! {
    P1PinCnfPull: u8,
    Disabled = 0,
    Pulldown = 1,
    Pullup = 3,
}

enum_bits! {
    P1PinCnfDrive: u8,
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
    P1PinCnfSense: u8,
    Disabled = 0,
    High = 2,
    Low = 3,
}
