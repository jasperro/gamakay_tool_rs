use std::{any::Any, fmt::Debug};

use strum::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum KeyCode {
    None = 0,
    Mouse = 1, // When using parameters, can have mouse effects
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10, // Or Fn when used with parameter 1
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Key1 = 30,
    Key2 = 31,
    Key3 = 32,
    Key4 = 33,
    Key5 = 34,
    Key6 = 35,
    Key7 = 36,
    Key8 = 37,
    Key9 = 38,
    Key0 = 39,
    Enter = 40,
    Esc = 41,
    BackSpace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equal = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    Semicolon = 51,
    Quote = 52,
    Backtick = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    Print = 70,
    ScrLk = 71,
    Pause = 72,
    Home = 74,
    PgUp = 75,
    Del = 76,
    End = 77,
    PgDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLk = 83,
    NumDiv = 84,
    NumMul = 85,
    NumMin = 86,
    NumAdd = 87,
    NumEnter = 88,
    Num1 = 89,
    Num2 = 90,
    Num3 = 91,
    Num4 = 92,
    Num5 = 93,
    Num6 = 94,
    Num7 = 95,
    Num8 = 96,
    Num9 = 97,
    Num0 = 98,
    NumPeriod = 99,
    Ctrl = 224,
    Shift = 225,
    Alt = 226,
    Win = 227,
    RightCtrl = 228,
    RightShift = 229,
    // Fn = 167837696, (0x0A010000)
}

#[derive(Debug, Clone, Copy)]
pub struct PositionedKey {
    pub key: KeyCode,
    pub matrix_index: u8,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub trait KeyboardLayoutExt {
    /// Find key by KeyCode
    fn find_by_key(&self, key: KeyCode) -> Option<&PositionedKey>;

    /// Find key by matrix position
    fn find_by_matrix(&self, index: u8) -> Option<&PositionedKey>;
}

impl KeyboardLayoutExt for KeyboardLayout {
    fn find_by_key(&self, key: KeyCode) -> Option<&PositionedKey> {
        self.iter().find(|pk| pk.key == key)
    }

    fn find_by_matrix(&self, index: u8) -> Option<&PositionedKey> {
        self.iter().find(|pk| pk.matrix_index == index)
    }
}

pub type KeyboardLayout = [PositionedKey];

pub const KEYBOARD_LAYOUT: &[PositionedKey; 101] = &[
    // Column 0
    PositionedKey {
        key: KeyCode::Esc,
        matrix_index: 0,
        x: 20,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Backtick,
        matrix_index: 1,
        x: 20,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Tab,
        matrix_index: 2,
        x: 20,
        y: -109,
        w: 57,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::CapsLock,
        matrix_index: 3,
        x: 20,
        y: -149,
        w: 67,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Shift,
        matrix_index: 4,
        x: 20,
        y: -189,
        w: 88,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Ctrl,
        matrix_index: 5,
        x: 20,
        y: -229,
        w: 51,
        h: 38,
    },
    // Column 1
    PositionedKey {
        key: KeyCode::Key1,
        matrix_index: 7,
        x: 61,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Q,
        matrix_index: 8,
        x: 80,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::A,
        matrix_index: 9,
        x: 90,
        y: -149,
        w: 38,
        h: 38,
    },
    // Column 2
    PositionedKey {
        key: KeyCode::F1,
        matrix_index: 12,
        x: 102,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key2,
        matrix_index: 13,
        x: 102,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::W,
        matrix_index: 14,
        x: 121,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::S,
        matrix_index: 15,
        x: 131,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Z,
        matrix_index: 16,
        x: 111,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Win,
        matrix_index: 17,
        x: 73,
        y: -229,
        w: 51,
        h: 38,
    },
    // Column 3
    PositionedKey {
        key: KeyCode::F2,
        matrix_index: 18,
        x: 143,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key3,
        matrix_index: 19,
        x: 142,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::E,
        matrix_index: 20,
        x: 162,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::D,
        matrix_index: 21,
        x: 171,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::X,
        matrix_index: 22,
        x: 152,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Alt,
        matrix_index: 23,
        x: 126,
        y: -229,
        w: 51,
        h: 38,
    },
    // Column 4
    PositionedKey {
        key: KeyCode::F3,
        matrix_index: 24,
        x: 183,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key4,
        matrix_index: 25,
        x: 183,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::R,
        matrix_index: 26,
        x: 202,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::F,
        matrix_index: 27,
        x: 212,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::C,
        matrix_index: 28,
        x: 193,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 5
    PositionedKey {
        key: KeyCode::F4,
        matrix_index: 30,
        x: 224,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key5,
        matrix_index: 31,
        x: 224,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::T,
        matrix_index: 32,
        x: 243,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::G,
        matrix_index: 33,
        x: 253,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::V,
        matrix_index: 34,
        x: 234,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 6
    PositionedKey {
        key: KeyCode::F5,
        matrix_index: 36,
        x: 284,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key6,
        matrix_index: 37,
        x: 265,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Y,
        matrix_index: 38,
        x: 284,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::H,
        matrix_index: 39,
        x: 294,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::B,
        matrix_index: 40,
        x: 275,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Space,
        matrix_index: 41,
        x: 179,
        y: -229,
        w: 243,
        h: 38,
    },
    // Column 7
    PositionedKey {
        key: KeyCode::F6,
        matrix_index: 42,
        x: 325,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key7,
        matrix_index: 43,
        x: 305,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::U,
        matrix_index: 44,
        x: 325,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::J,
        matrix_index: 45,
        x: 335,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::N,
        matrix_index: 46,
        x: 315,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 8
    PositionedKey {
        key: KeyCode::F7,
        matrix_index: 48,
        x: 366,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key8,
        matrix_index: 49,
        x: 346,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::I,
        matrix_index: 50,
        x: 366,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::K,
        matrix_index: 51,
        x: 375,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::M,
        matrix_index: 52,
        x: 356,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 9
    PositionedKey {
        key: KeyCode::F8,
        matrix_index: 54,
        x: 407,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key9,
        matrix_index: 55,
        x: 387,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::O,
        matrix_index: 56,
        x: 407,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::L,
        matrix_index: 57,
        x: 416,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Comma,
        matrix_index: 58,
        x: 397,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 10
    PositionedKey {
        key: KeyCode::F9,
        matrix_index: 60,
        x: 467,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Key0,
        matrix_index: 61,
        x: 428,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::P,
        matrix_index: 62,
        x: 447,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Semicolon,
        matrix_index: 63,
        x: 457,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Period,
        matrix_index: 64,
        x: 438,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        // TODO: find a way to use the ComplexAction Fn (or another way)
        key: KeyCode::None,
        matrix_index: 65,
        x: 424,
        y: -229,
        w: 51,
        h: 38,
    },
    // Column 11
    PositionedKey {
        key: KeyCode::F10,
        matrix_index: 66,
        x: 508,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Minus,
        matrix_index: 67,
        x: 468,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::LeftBracket,
        matrix_index: 68,
        x: 488,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Quote,
        matrix_index: 69,
        x: 498,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Slash,
        matrix_index: 70,
        x: 479,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::RightCtrl,
        matrix_index: 71,
        x: 477,
        y: -229,
        w: 51,
        h: 38,
    },
    // Column 12
    PositionedKey {
        key: KeyCode::F11,
        matrix_index: 72,
        x: 548,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Equal,
        matrix_index: 73,
        x: 509,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::RightBracket,
        matrix_index: 74,
        x: 529,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::RightShift,
        matrix_index: 76,
        x: 519,
        y: -189,
        w: 67,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Left,
        matrix_index: 77,
        x: 548,
        y: -229,
        w: 38,
        h: 38,
    },
    // Column 13
    PositionedKey {
        key: KeyCode::F12,
        matrix_index: 78,
        x: 589,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::BackSpace,
        matrix_index: 79,
        x: 550,
        y: -69,
        w: 77,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Backslash,
        matrix_index: 80,
        x: 570,
        y: -109,
        w: 57,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Enter,
        matrix_index: 81,
        x: 539,
        y: -149,
        w: 88,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Up,
        matrix_index: 82,
        x: 589,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Down,
        matrix_index: 83,
        x: 589,
        y: -229,
        w: 38,
        h: 38,
    },
    // Column 14 (Navigation)
    PositionedKey {
        key: KeyCode::Home,
        matrix_index: 84,
        x: 641,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::PgUp,
        matrix_index: 85,
        x: 641,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::PgDown,
        matrix_index: 86,
        x: 641,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::End,
        matrix_index: 87,
        x: 641,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Right,
        matrix_index: 89,
        x: 630,
        y: -229,
        w: 38,
        h: 38,
    },
    // Column 15 (Numpad Links)
    PositionedKey {
        key: KeyCode::Pause,
        matrix_index: 90,
        x: 691,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumLk,
        matrix_index: 91,
        x: 691,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num7,
        matrix_index: 92,
        x: 691,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num4,
        matrix_index: 93,
        x: 691,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num1,
        matrix_index: 94,
        x: 691,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num0,
        matrix_index: 95,
        x: 691,
        y: -229,
        w: 77,
        h: 38,
    },
    // Column 16 (Numpad Midden)
    PositionedKey {
        key: KeyCode::ScrLk,
        matrix_index: 96,
        x: 732,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumDiv,
        matrix_index: 97,
        x: 732,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num8,
        matrix_index: 98,
        x: 732,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num5,
        matrix_index: 99,
        x: 732,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num2,
        matrix_index: 100,
        x: 732,
        y: -189,
        w: 38,
        h: 38,
    },
    // Column 17 (Numpad Rechts)
    PositionedKey {
        key: KeyCode::Print,
        matrix_index: 102,
        x: 773,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumMul,
        matrix_index: 103,
        x: 773,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num9,
        matrix_index: 104,
        x: 773,
        y: -109,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num6,
        matrix_index: 105,
        x: 773,
        y: -149,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::Num3,
        matrix_index: 106,
        x: 773,
        y: -189,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumPeriod,
        matrix_index: 107,
        x: 773,
        y: -229,
        w: 38,
        h: 38,
    },
    // Column 18 (Numpad Rand)
    PositionedKey {
        key: KeyCode::Del,
        matrix_index: 108,
        x: 814,
        y: -20,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumMin,
        matrix_index: 109,
        x: 814,
        y: -69,
        w: 38,
        h: 38,
    },
    PositionedKey {
        key: KeyCode::NumAdd,
        matrix_index: 110,
        x: 814,
        y: -109,
        w: 38,
        h: 78,
    },
    PositionedKey {
        key: KeyCode::NumEnter,
        matrix_index: 112,
        x: 814,
        y: -189,
        w: 38,
        h: 77,
    },
];

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum ComplexAction {
    // Part 1: Mouse & Movement
    MouseLeft,
    MouseRight,
    MouseMiddle,
    Forward,
    Back,
    Dpi,
    WheelForward,
    WheelBack,
    WheelLeft,
    WheelRight,
    ScrollUp,
    ScrollDown,
    XUp,
    XDown,
    YUp,
    YDown,
    // Part 2: Media & System
    PreviousTrack,
    NextTrack,
    Stop,
    PlayPause,
    Player,
    Firepower,
    Mute,
    VolumeDown,
    VolumeUp,
    Calculator,
    FnSmall,
    Mail,
    MyComputer,
    Search,
    Home,
    BrightnessDecrease,
    BrightnessIncrease,
    SiriActivation,
    RecoilControl,
    RecoilControlContinuous,
    RecoilControlToggle,
    WeaponSwitchIncrease,
    WeaponSwitchDecrease,
    WeaponSwitchLoop,
    SystemPower,
    SystemSleep,
    WakeUp,
    // Part 3: Symbols & Shortcuts
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SwitchInputMethod,
    ZoomOut,
    ZoomIn,
    AiFunction,
    MicrophoneSwitch,
    BackAlternative,
    FnLockScreen,
    LockScreen,
    RightFn,
    Fn,
    VolKbBrightness,
    VolKbBrightness2,
    VolKbBrightness3,
    Refresh,
    ProgramOpen1,
    ProgramOpen2,
    ProgramOpen3,
    DpiLoop,
    DpiPlus,
    DpiMinus,
    DpiShift,
    ProfileValue,
    ProfilePlus,
    ProfileMinus,
    ProfileLoop,
    ProfileExchange,
    ProfileExchange2,
    ProfileExchange3,
    ProfileExchange4,
    ProfileExchange5,
    OpenWebsite,
    WinE,
    WinTab,
    WinD,
}

impl ComplexAction {
    pub fn values(&self) -> [u8; 4] {
        match self {
            Self::MouseLeft => [1, 0, 240, 0],
            Self::MouseRight => [1, 0, 241, 0],
            Self::MouseMiddle => [1, 0, 242, 0],
            Self::Forward => [1, 0, 244, 0],
            Self::Back => [1, 0, 243, 0],
            Self::Dpi => [20, 0, 0, 0],
            Self::WheelForward => [1, 0, 247, 0],
            Self::WheelBack => [1, 0, 248, 0],
            Self::WheelLeft => [1, 0, 245, 0],
            Self::WheelRight => [1, 0, 246, 0],
            Self::ScrollUp => [1, 0, 245, 1],
            Self::ScrollDown => [1, 0, 245, 255],
            Self::XUp => [1, 0, 246, 251],
            Self::XDown => [1, 0, 246, 5],
            Self::YUp => [1, 0, 247, 251],
            Self::YDown => [1, 0, 247, 5],
            Self::PreviousTrack => [3, 0, 182, 0],
            Self::NextTrack => [3, 0, 181, 0],
            Self::Stop => [3, 0, 183, 0],
            Self::PlayPause => [3, 0, 205, 0],
            Self::Player => [3, 0, 131, 1],
            Self::Firepower => [11, 0, 0, 0],
            Self::Mute => [3, 0, 226, 0],
            Self::VolumeDown => [3, 0, 234, 0],
            Self::VolumeUp => [3, 0, 233, 0],
            Self::Calculator => [3, 0, 146, 1],
            Self::FnSmall => [10, 1, 0, 0],
            Self::Mail => [3, 0, 138, 1],
            Self::MyComputer => [3, 0, 148, 1],
            Self::Search => [3, 0, 33, 2],
            Self::Home => [3, 0, 35, 2],
            Self::BrightnessDecrease => [3, 0, 112, 0],
            Self::BrightnessIncrease => [3, 0, 111, 0],
            Self::SiriActivation => [18, 0, 227, 44],
            Self::RecoilControl => [22, 4, 0, 0],
            Self::RecoilControlContinuous => [22, 6, 0, 0],
            Self::RecoilControlToggle => [22, 7, 0, 0],
            Self::WeaponSwitchIncrease => [22, 1, 0, 0],
            Self::WeaponSwitchDecrease => [22, 2, 0, 0],
            Self::WeaponSwitchLoop => [22, 3, 0, 0],
            Self::SystemPower => [2, 129, 0, 0],
            Self::SystemSleep => [2, 130, 0, 0],
            Self::WakeUp => [2, 131, 0, 0],
            Self::LeftParen => [0, 0, 229, 38],
            Self::RightParen => [0, 0, 229, 39],
            Self::LeftBrace => [0, 225, 47, 0],
            Self::RightBrace => [0, 225, 48, 0],
            Self::SwitchInputMethod => [0, 224, 44, 0],
            Self::ZoomOut => [0, 0, 227, 45],
            Self::ZoomIn => [0, 0, 227, 46],
            Self::AiFunction => [0, 227, 225, 114],
            Self::MicrophoneSwitch => [6, 128, 0, 0],
            Self::BackAlternative => [3, 0, 36, 2],
            Self::FnLockScreen => [10, 13, 0, 0],
            Self::LockScreen => [0, 0, 227, 15],
            Self::RightFn => [10, 1, 1, 0],
            Self::Fn => [10, 1, 0, 0],
            Self::VolKbBrightness => [10, 15, 0, 0],
            Self::VolKbBrightness2 => [10, 13, 0, 0],
            Self::VolKbBrightness3 => [10, 14, 0, 0],
            Self::Refresh => [3, 0, 39, 2],
            Self::ProgramOpen1 => [6, 128, 0, 1],
            Self::ProgramOpen2 => [6, 128, 0, 2],
            Self::ProgramOpen3 => [6, 128, 0, 3],
            Self::DpiLoop => [20, 0, 0, 0],
            Self::DpiPlus => [20, 0, 1, 0],
            Self::DpiMinus => [20, 0, 2, 0],
            Self::DpiShift => [20, 0, 4, 0],
            Self::ProfileValue => [8, 0, 0, 0],
            Self::ProfilePlus => [8, 0, 1, 0],
            Self::ProfileMinus => [8, 0, 2, 0],
            Self::ProfileLoop => [8, 0, 3, 0],
            Self::ProfileExchange => [8, 0, 4, 0],
            Self::ProfileExchange2 => [8, 0, 4, 1],
            Self::ProfileExchange3 => [8, 0, 4, 2],
            Self::ProfileExchange4 => [8, 0, 4, 3],
            Self::ProfileExchange5 => [8, 0, 4, 4],
            Self::OpenWebsite => [18, 0, 0, 0],
            Self::WinE => [0, 0, 227, 8],
            Self::WinTab => [0, 0, 227, 43],
            Self::WinD => [0, 0, 227, 7],
        }
    }
}

pub trait KeyAction: Debug + DynEq {
    fn to_bytes(&self) -> [u8; 4];
    fn legend(&self) -> String;
}

impl KeyAction for KeyCode {
    fn to_bytes(&self) -> [u8; 4] {
        [0, 0, 0, *self as u8]
    }

    fn legend(&self) -> String {
        match self {
            KeyCode::Backtick => "`".to_string(),
            KeyCode::Minus => "-".to_string(),
            KeyCode::Equal => "=".to_string(),
            KeyCode::LeftBracket => "[".to_string(),
            KeyCode::RightBracket => "]".to_string(),
            KeyCode::Backslash => "\\".to_string(),
            KeyCode::Semicolon => ";".to_string(),
            KeyCode::Quote => "'".to_string(),
            KeyCode::Comma => ",".to_string(),
            KeyCode::Period => ".".to_string(),
            KeyCode::Slash => "/".to_string(),
            KeyCode::Key0 | KeyCode::Num0 => "0".to_string(),
            KeyCode::Key1 | KeyCode::Num1 => "1".to_string(),
            KeyCode::Key2 | KeyCode::Num2 => "2".to_string(),
            KeyCode::Key3 | KeyCode::Num3 => "3".to_string(),
            KeyCode::Key4 | KeyCode::Num4 => "4".to_string(),
            KeyCode::Key5 | KeyCode::Num5 => "5".to_string(),
            KeyCode::Key6 | KeyCode::Num6 => "6".to_string(),
            KeyCode::Key7 | KeyCode::Num7 => "7".to_string(),
            KeyCode::Key8 | KeyCode::Num8 => "8".to_string(),
            KeyCode::Key9 | KeyCode::Num9 => "9".to_string(),
            KeyCode::NumEnter => "↵".to_string(),
            KeyCode::NumAdd => "+".to_string(),
            KeyCode::NumMin => "-".to_string(),
            KeyCode::NumDiv => "÷".to_string(),
            KeyCode::NumMul => "×".to_string(),
            KeyCode::NumPeriod => ".".to_string(),
            KeyCode::RightCtrl => "Ctrl".to_string(),
            KeyCode::RightShift => "Shift".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

// Implementation for ExtraKey (cannot be combined)
impl KeyAction for ComplexAction {
    fn to_bytes(&self) -> [u8; 4] {
        self.values()
    }

    fn legend(&self) -> String {
        format!("{:?}", self)
    }
}

// Struct for Key Combinations (e.g., Ctrl + Alt + Delete)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyCombo<const N: usize>
where
    // KeyCombo with one key does not exist.
    [(); N + 1]:,
{
    pub keys: [KeyCode; N],
}

pub trait Combinable {
    type Next;
    fn with(self, next: KeyCode) -> Self::Next;
}

// Base KeyCode to KeyCombo<2>
impl Combinable for KeyCode {
    type Next = KeyCombo<2>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo { keys: [self, next] }
    }
}

impl Combinable for KeyCombo<2> {
    type Next = KeyCombo<3>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo {
            keys: [self.keys[0], self.keys[1], next],
        }
    }
}

impl Combinable for KeyCombo<3> {
    type Next = KeyCombo<4>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo {
            keys: [self.keys[0], self.keys[1], self.keys[2], next],
        }
    }
}

impl<const N: usize> KeyAction for KeyCombo<N>
where
    [(); N + 1]:,
{
    fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];

        // Because we know that N is at most 4 (due to our Combinable constraint),
        // we fill the array from the right (index 3).
        // i = 0 -> bytes[3], i = 1 -> bytes[2], etc.
        for i in 0..N {
            // We pakken de toetsen van achter naar voren uit de combo
            let key = self.keys[N - 1 - i];
            bytes[3 - i] = key as u8;
        }
        bytes
    }

    fn legend(&self) -> String {
        // We map each KeyCode in the array to its own legend() String
        // and group them together with " + ".
        self.keys
            .iter()
            .map(|k| k.legend())
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

pub trait DynEq: Any {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: PartialEq + Any> DynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        // First check if the other object is the same concrete type
        if let Some(other_concrete) = other.as_any().downcast_ref::<Self>() {
            // If they match, use the concrete PartialEq implementation
            self == other_concrete
        } else {
            // If types differ, they are not equal
            false
        }
    }
}

impl PartialEq for dyn KeyAction {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}
