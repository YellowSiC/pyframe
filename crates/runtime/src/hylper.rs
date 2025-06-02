// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use image::ImageFormat;
use muda;
use muda::accelerator::Code as MudaCode;
use muda::accelerator::Modifiers as MudaModifiers;
use muda::NativeIcon;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::read, path::PathBuf};

use crate::options::menu::CheckMenuItem;
use crate::options::menu::IconMenuItem;
use crate::options::menu::MenuItem;
use crate::options::menu::PredefinedMenuItem;
use crate::options::menu::Submenu;
use crate::options::menu::SystemTray;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AcceleratorCode {
    #[serde(rename = "backquote")]
    Backquote,
    #[serde(rename = "backslash")]
    Backslash,
    #[serde(rename = "bracketLeft")]
    BracketLeft,
    #[serde(rename = "bracketRight")]
    BracketRight,
    #[serde(rename = "comma")]
    Comma,
    #[serde(rename = "digit0")]
    Digit0,
    #[serde(rename = "digit1")]
    Digit1,
    #[serde(rename = "digit2")]
    Digit2,
    #[serde(rename = "digit3")]
    Digit3,
    #[serde(rename = "digit4")]
    Digit4,
    #[serde(rename = "digit5")]
    Digit5,
    #[serde(rename = "digit6")]
    Digit6,
    #[serde(rename = "digit7")]
    Digit7,
    #[serde(rename = "digit8")]
    Digit8,
    #[serde(rename = "digit9")]
    Digit9,
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "intlBackslash")]
    IntlBackslash,
    #[serde(rename = "intlRo")]
    IntlRo,
    #[serde(rename = "intlYen")]
    IntlYen,
    #[serde(rename = "keyA")]
    KeyA,
    #[serde(rename = "keyB")]
    KeyB,
    #[serde(rename = "keyC")]
    KeyC,
    #[serde(rename = "keyD")]
    KeyD,
    #[serde(rename = "keyE")]
    KeyE,
    #[serde(rename = "keyF")]
    KeyF,
    #[serde(rename = "keyG")]
    KeyG,
    #[serde(rename = "keyH")]
    KeyH,
    #[serde(rename = "keyI")]
    KeyI,
    #[serde(rename = "keyJ")]
    KeyJ,
    #[serde(rename = "keyK")]
    KeyK,
    #[serde(rename = "keyL")]
    KeyL,
    #[serde(rename = "keyM")]
    KeyM,
    #[serde(rename = "keyN")]
    KeyN,
    #[serde(rename = "keyO")]
    KeyO,
    #[serde(rename = "keyP")]
    KeyP,
    #[serde(rename = "keyQ")]
    KeyQ,
    #[serde(rename = "keyR")]
    KeyR,
    #[serde(rename = "keyS")]
    KeyS,
    #[serde(rename = "keyT")]
    KeyT,
    #[serde(rename = "keyU")]
    KeyU,
    #[serde(rename = "keyV")]
    KeyV,
    #[serde(rename = "keyW")]
    KeyW,
    #[serde(rename = "keyX")]
    KeyX,
    #[serde(rename = "keyY")]
    KeyY,
    #[serde(rename = "keyZ")]
    KeyZ,
    #[serde(rename = "minus")]
    Minus,
    #[serde(rename = "period")]
    Period,
    #[serde(rename = "quote")]
    Quote,
    #[serde(rename = "semicolon")]
    Semicolon,
    #[serde(rename = "slash")]
    Slash,
    #[serde(rename = "altLeft")]
    AltLeft,
    #[serde(rename = "altRight")]
    AltRight,
    #[serde(rename = "backspace")]
    Backspace,
    #[serde(rename = "capsLock")]
    CapsLock,
    #[serde(rename = "contextMenu")]
    ContextMenu,
    #[serde(rename = "controlLeft")]
    ControlLeft,
    #[serde(rename = "controlRight")]
    ControlRight,
    #[serde(rename = "enter")]
    Enter,
    #[serde(rename = "metaLeft")]
    MetaLeft,
    #[serde(rename = "metaRight")]
    MetaRight,
    #[serde(rename = "shiftLeft")]
    ShiftLeft,
    #[serde(rename = "shiftRight")]
    ShiftRight,
    #[serde(rename = "space")]
    Space,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "convert")]
    Convert,
    #[serde(rename = "kanaMode")]
    KanaMode,
    #[serde(rename = "lang1")]
    Lang1,
    #[serde(rename = "lang2")]
    Lang2,
    #[serde(rename = "lang3")]
    Lang3,
    #[serde(rename = "lang4")]
    Lang4,
    #[serde(rename = "lang5")]
    Lang5,
    #[serde(rename = "nonConvert")]
    NonConvert,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "help")]
    Help,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "insert")]
    Insert,
    #[serde(rename = "pageDown")]
    PageDown,
    #[serde(rename = "pageUp")]
    PageUp,
    #[serde(rename = "arrowDown")]
    ArrowDown,
    #[serde(rename = "arrowLeft")]
    ArrowLeft,
    #[serde(rename = "arrowRight")]
    ArrowRight,
    #[serde(rename = "arrowUp")]
    ArrowUp,
    #[serde(rename = "numLock")]
    NumLock,
    #[serde(rename = "numpad0")]
    Numpad0,
    #[serde(rename = "numpad1")]
    Numpad1,
    #[serde(rename = "numpad2")]
    Numpad2,
    #[serde(rename = "numpad3")]
    Numpad3,
    #[serde(rename = "numpad4")]
    Numpad4,
    #[serde(rename = "numpad5")]
    Numpad5,
    #[serde(rename = "numpad6")]
    Numpad6,
    #[serde(rename = "numpad7")]
    Numpad7,
    #[serde(rename = "numpad8")]
    Numpad8,
    #[serde(rename = "numpad9")]
    Numpad9,
    #[serde(rename = "numpadAdd")]
    NumpadAdd,
    #[serde(rename = "numpadBackspace")]
    NumpadBackspace,
    #[serde(rename = "numpadClear")]
    NumpadClear,
    #[serde(rename = "numpadClearEntry")]
    NumpadClearEntry,
    #[serde(rename = "numpadComma")]
    NumpadComma,
    #[serde(rename = "numpadDecimal")]
    NumpadDecimal,
    #[serde(rename = "numpadDivide")]
    NumpadDivide,
    #[serde(rename = "numpadEnter")]
    NumpadEnter,
    #[serde(rename = "numpadEqual")]
    NumpadEqual,
    #[serde(rename = "numpadHash")]
    NumpadHash,
    #[serde(rename = "numpadMemoryAdd")]
    NumpadMemoryAdd,
    #[serde(rename = "numpadMemoryClear")]
    NumpadMemoryClear,
    #[serde(rename = "numpadMemoryRecall")]
    NumpadMemoryRecall,
    #[serde(rename = "numpadMemoryStore")]
    NumpadMemoryStore,
    #[serde(rename = "numpadMemorySubtract")]
    NumpadMemorySubtract,
    #[serde(rename = "numpadMultiply")]
    NumpadMultiply,
    #[serde(rename = "numpadParenLeft")]
    NumpadParenLeft,
    #[serde(rename = "numpadParenRight")]
    NumpadParenRight,
    #[serde(rename = "numpadStar")]
    NumpadStar,
    #[serde(rename = "numpadSubtract")]
    NumpadSubtract,
    #[serde(rename = "escape")]
    Escape,
    #[serde(rename = "fn")]
    Fn,
    #[serde(rename = "fnLock")]
    FnLock,
    #[serde(rename = "printScreen")]
    PrintScreen,
    #[serde(rename = "scrollLock")]
    ScrollLock,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "browserBack")]
    BrowserBack,
    #[serde(rename = "browserFavorites")]
    BrowserFavorites,
    #[serde(rename = "browserForward")]
    BrowserForward,
    #[serde(rename = "browserHome")]
    BrowserHome,
    #[serde(rename = "browserRefresh")]
    BrowserRefresh,
    #[serde(rename = "browserSearch")]
    BrowserSearch,
    #[serde(rename = "browserStop")]
    BrowserStop,
    #[serde(rename = "eject")]
    Eject,
    #[serde(rename = "launchApp1")]
    LaunchApp1,
    #[serde(rename = "launchApp2")]
    LaunchApp2,
    #[serde(rename = "launchMail")]
    LaunchMail,
    #[serde(rename = "mediaPlayPause")]
    MediaPlayPause,
    #[serde(rename = "mediaSelect")]
    MediaSelect,
    #[serde(rename = "mediaStop")]
    MediaStop,
    #[serde(rename = "mediaTrackNext")]
    MediaTrackNext,
    #[serde(rename = "mediaTrackPrevious")]
    MediaTrackPrevious,
    #[serde(rename = "power")]
    Power,
    #[serde(rename = "sleep")]
    Sleep,
    #[serde(rename = "audioVolumeDown")]
    AudioVolumeDown,
    #[serde(rename = "audioVolumeMute")]
    AudioVolumeMute,
    #[serde(rename = "audioVolumeUp")]
    AudioVolumeUp,
    #[serde(rename = "wakeUp")]
    WakeUp,
    #[serde(rename = "hyper")]
    Hyper,
    #[serde(rename = "super")]
    Super,
    #[serde(rename = "turbo")]
    Turbo,
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "suspend")]
    Suspend,
    #[serde(rename = "again")]
    Again,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "find")]
    Find,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "props")]
    Props,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "hiragana")]
    Hiragana,
    #[serde(rename = "katakana")]
    Katakana,
    #[serde(rename = "unidentified")]
    Unidentified,
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
    #[serde(rename = "f3")]
    F3,
    #[serde(rename = "f4")]
    F4,
    #[serde(rename = "f5")]
    F5,
    #[serde(rename = "f6")]
    F6,
    #[serde(rename = "f7")]
    F7,
    #[serde(rename = "f8")]
    F8,
    #[serde(rename = "f9")]
    F9,
    #[serde(rename = "f10")]
    F10,
    #[serde(rename = "f11")]
    F11,
    #[serde(rename = "f12")]
    F12,
    #[serde(rename = "f13")]
    F13,
    #[serde(rename = "f14")]
    F14,
    #[serde(rename = "f15")]
    F15,
    #[serde(rename = "f16")]
    F16,
    #[serde(rename = "f17")]
    F17,
    #[serde(rename = "f18")]
    F18,
    #[serde(rename = "f19")]
    F19,
    #[serde(rename = "f20")]
    F20,
    #[serde(rename = "f21")]
    F21,
    #[serde(rename = "f22")]
    F22,
    #[serde(rename = "f23")]
    F23,
    #[serde(rename = "f24")]
    F24,
    #[serde(rename = "f25")]
    F25,
    #[serde(rename = "f26")]
    F26,
    #[serde(rename = "f27")]
    F27,
    #[serde(rename = "f28")]
    F28,
    #[serde(rename = "f29")]
    F29,
    #[serde(rename = "f30")]
    F30,
    #[serde(rename = "f31")]
    F31,
    #[serde(rename = "f32")]
    F32,
    #[serde(rename = "f33")]
    F33,
    #[serde(rename = "f34")]
    F34,
    #[serde(rename = "f35")]
    F35,
    #[serde(rename = "brightnessDown")]
    BrightnessDown,
    #[serde(rename = "brightnessUp")]
    BrightnessUp,
    #[serde(rename = "displayToggleIntExt")]
    DisplayToggleIntExt,
    #[serde(rename = "keyboardLayoutSelect")]
    KeyboardLayoutSelect,
    #[serde(rename = "launchAssistant")]
    LaunchAssistant,
    #[serde(rename = "launchControlPanel")]
    LaunchControlPanel,
    #[serde(rename = "launchScreenSaver")]
    LaunchScreenSaver,
    #[serde(rename = "mailForward")]
    MailForward,
    #[serde(rename = "mailReply")]
    MailReply,
    #[serde(rename = "mailSend")]
    MailSend,
    #[serde(rename = "mediaFastForward")]
    MediaFastForward,
    #[serde(rename = "mediaPause")]
    MediaPause,
    #[serde(rename = "mediaPlay")]
    MediaPlay,
    #[serde(rename = "mediaRecord")]
    MediaRecord,
    #[serde(rename = "mediaRewind")]
    MediaRewind,
    #[serde(rename = "microphoneMuteToggle")]
    MicrophoneMuteToggle,
    #[serde(rename = "privacyScreenToggle")]
    PrivacyScreenToggle,
    #[serde(rename = "selectTask")]
    SelectTask,
    #[serde(rename = "showAllWindows")]
    ShowAllWindows,
    #[serde(rename = "zoomToggle")]
    ZoomToggle,
}

impl From<AcceleratorCode> for MudaCode {
    fn from(code: AcceleratorCode) -> Self {
        use AcceleratorCode::*;
        match code {
            Backquote => MudaCode::Backquote,
            Backslash => MudaCode::Backslash,
            BracketLeft => MudaCode::BracketLeft,
            BracketRight => MudaCode::BracketRight,
            Comma => MudaCode::Comma,
            Digit0 => MudaCode::Digit0,
            Digit1 => MudaCode::Digit1,
            Digit2 => MudaCode::Digit2,
            Digit3 => MudaCode::Digit3,
            Digit4 => MudaCode::Digit4,
            Digit5 => MudaCode::Digit5,
            Digit6 => MudaCode::Digit6,
            Digit7 => MudaCode::Digit7,
            Digit8 => MudaCode::Digit8,
            Digit9 => MudaCode::Digit9,
            Equal => MudaCode::Equal,
            IntlBackslash => MudaCode::IntlBackslash,
            IntlRo => MudaCode::IntlRo,
            IntlYen => MudaCode::IntlYen,
            KeyA => MudaCode::KeyA,
            KeyB => MudaCode::KeyB,
            KeyC => MudaCode::KeyC,
            KeyD => MudaCode::KeyD,
            KeyE => MudaCode::KeyE,
            KeyF => MudaCode::KeyF,
            KeyG => MudaCode::KeyG,
            KeyH => MudaCode::KeyH,
            KeyI => MudaCode::KeyI,
            KeyJ => MudaCode::KeyJ,
            KeyK => MudaCode::KeyK,
            KeyL => MudaCode::KeyL,
            KeyM => MudaCode::KeyM,
            KeyN => MudaCode::KeyN,
            KeyO => MudaCode::KeyO,
            KeyP => MudaCode::KeyP,
            KeyQ => MudaCode::KeyQ,
            KeyR => MudaCode::KeyR,
            KeyS => MudaCode::KeyS,
            KeyT => MudaCode::KeyT,
            KeyU => MudaCode::KeyU,
            KeyV => MudaCode::KeyV,
            KeyW => MudaCode::KeyW,
            KeyX => MudaCode::KeyX,
            KeyY => MudaCode::KeyY,
            KeyZ => MudaCode::KeyZ,
            Minus => MudaCode::Minus,
            Period => MudaCode::Period,
            Quote => MudaCode::Quote,
            Semicolon => MudaCode::Semicolon,
            Slash => MudaCode::Slash,
            AltLeft => MudaCode::AltLeft,
            AltRight => MudaCode::AltRight,
            Backspace => MudaCode::Backspace,
            CapsLock => MudaCode::CapsLock,
            ContextMenu => MudaCode::ContextMenu,
            ControlLeft => MudaCode::ControlLeft,
            ControlRight => MudaCode::ControlRight,
            Enter => MudaCode::Enter,
            MetaLeft => MudaCode::MetaLeft,
            MetaRight => MudaCode::MetaRight,
            ShiftLeft => MudaCode::ShiftLeft,
            ShiftRight => MudaCode::ShiftRight,
            Space => MudaCode::Space,
            Tab => MudaCode::Tab,
            Convert => MudaCode::Convert,
            KanaMode => MudaCode::KanaMode,
            Lang1 => MudaCode::Lang1,
            Lang2 => MudaCode::Lang2,
            Lang3 => MudaCode::Lang3,
            Lang4 => MudaCode::Lang4,
            Lang5 => MudaCode::Lang5,
            NonConvert => MudaCode::NonConvert,
            Delete => MudaCode::Delete,
            End => MudaCode::End,
            Help => MudaCode::Help,
            Home => MudaCode::Home,
            Insert => MudaCode::Insert,
            PageDown => MudaCode::PageDown,
            PageUp => MudaCode::PageUp,
            ArrowDown => MudaCode::ArrowDown,
            ArrowLeft => MudaCode::ArrowLeft,
            ArrowRight => MudaCode::ArrowRight,
            ArrowUp => MudaCode::ArrowUp,
            NumLock => MudaCode::NumLock,
            Numpad0 => MudaCode::Numpad0,
            Numpad1 => MudaCode::Numpad1,
            Numpad2 => MudaCode::Numpad2,
            Numpad3 => MudaCode::Numpad3,
            Numpad4 => MudaCode::Numpad4,
            Numpad5 => MudaCode::Numpad5,
            Numpad6 => MudaCode::Numpad6,
            Numpad7 => MudaCode::Numpad7,
            Numpad8 => MudaCode::Numpad8,
            Numpad9 => MudaCode::Numpad9,
            NumpadAdd => MudaCode::NumpadAdd,
            NumpadBackspace => MudaCode::NumpadBackspace,
            NumpadClear => MudaCode::NumpadClear,
            NumpadClearEntry => MudaCode::NumpadClearEntry,
            NumpadComma => MudaCode::NumpadComma,
            NumpadDecimal => MudaCode::NumpadDecimal,
            NumpadDivide => MudaCode::NumpadDivide,
            NumpadEnter => MudaCode::NumpadEnter,
            NumpadEqual => MudaCode::NumpadEqual,
            NumpadHash => MudaCode::NumpadHash,
            NumpadMemoryAdd => MudaCode::NumpadMemoryAdd,
            NumpadMemoryClear => MudaCode::NumpadMemoryClear,
            NumpadMemoryRecall => MudaCode::NumpadMemoryRecall,
            NumpadMemoryStore => MudaCode::NumpadMemoryStore,
            NumpadMemorySubtract => MudaCode::NumpadMemorySubtract,
            NumpadMultiply => MudaCode::NumpadMultiply,
            NumpadParenLeft => MudaCode::NumpadParenLeft,
            NumpadParenRight => MudaCode::NumpadParenRight,
            NumpadStar => MudaCode::NumpadStar,
            NumpadSubtract => MudaCode::NumpadSubtract,
            Escape => MudaCode::Escape,
            Fn => MudaCode::Fn,
            FnLock => MudaCode::FnLock,
            PrintScreen => MudaCode::PrintScreen,
            ScrollLock => MudaCode::ScrollLock,
            Pause => MudaCode::Pause,
            BrowserBack => MudaCode::BrowserBack,
            BrowserFavorites => MudaCode::BrowserFavorites,
            BrowserForward => MudaCode::BrowserForward,
            BrowserHome => MudaCode::BrowserHome,
            BrowserRefresh => MudaCode::BrowserRefresh,
            BrowserSearch => MudaCode::BrowserSearch,
            BrowserStop => MudaCode::BrowserStop,
            Eject => MudaCode::Eject,
            LaunchApp1 => MudaCode::LaunchApp1,
            LaunchApp2 => MudaCode::LaunchApp2,
            LaunchMail => MudaCode::LaunchMail,
            MediaPlayPause => MudaCode::MediaPlayPause,
            MediaSelect => MudaCode::MediaSelect,
            MediaStop => MudaCode::MediaStop,
            MediaTrackNext => MudaCode::MediaTrackNext,
            MediaTrackPrevious => MudaCode::MediaTrackPrevious,
            Power => MudaCode::Power,
            Sleep => MudaCode::Sleep,
            AudioVolumeDown => MudaCode::AudioVolumeDown,
            AudioVolumeMute => MudaCode::AudioVolumeMute,
            AudioVolumeUp => MudaCode::AudioVolumeUp,
            WakeUp => MudaCode::WakeUp,
            Hyper => MudaCode::Hyper,
            Super => MudaCode::Super,
            Turbo => MudaCode::Turbo,
            Abort => MudaCode::Abort,
            Resume => MudaCode::Resume,
            Suspend => MudaCode::Suspend,
            Again => MudaCode::Again,
            Copy => MudaCode::Copy,
            Cut => MudaCode::Cut,
            Find => MudaCode::Find,
            Open => MudaCode::Open,
            Paste => MudaCode::Paste,
            Props => MudaCode::Props,
            Select => MudaCode::Select,
            Undo => MudaCode::Undo,
            Hiragana => MudaCode::Hiragana,
            Katakana => MudaCode::Katakana,
            Unidentified => MudaCode::Unidentified,
            F1 => MudaCode::F1,
            F2 => MudaCode::F2,
            F3 => MudaCode::F3,
            F4 => MudaCode::F4,
            F5 => MudaCode::F5,
            F6 => MudaCode::F6,
            F7 => MudaCode::F7,
            F8 => MudaCode::F8,
            F9 => MudaCode::F9,
            F10 => MudaCode::F10,
            F11 => MudaCode::F11,
            F12 => MudaCode::F12,
            F13 => MudaCode::F13,
            F14 => MudaCode::F14,
            F15 => MudaCode::F15,
            F16 => MudaCode::F16,
            F17 => MudaCode::F17,
            F18 => MudaCode::F18,
            F19 => MudaCode::F19,
            F20 => MudaCode::F20,
            F21 => MudaCode::F21,
            F22 => MudaCode::F22,
            F23 => MudaCode::F23,
            F24 => MudaCode::F24,
            F25 => MudaCode::F25,
            F26 => MudaCode::F26,
            F27 => MudaCode::F27,
            F28 => MudaCode::F28,
            F29 => MudaCode::F29,
            F30 => MudaCode::F30,
            F31 => MudaCode::F31,
            F32 => MudaCode::F32,
            F33 => MudaCode::F33,
            F34 => MudaCode::F34,
            F35 => MudaCode::F35,
            BrightnessDown => MudaCode::BrightnessDown,
            BrightnessUp => MudaCode::BrightnessUp,
            DisplayToggleIntExt => MudaCode::DisplayToggleIntExt,
            KeyboardLayoutSelect => MudaCode::KeyboardLayoutSelect,
            LaunchAssistant => MudaCode::LaunchAssistant,
            LaunchControlPanel => MudaCode::LaunchControlPanel,
            LaunchScreenSaver => MudaCode::LaunchScreenSaver,
            MailForward => MudaCode::MailForward,
            MailReply => MudaCode::MailReply,
            MailSend => MudaCode::MailSend,
            MediaFastForward => MudaCode::MediaFastForward,
            MediaPause => MudaCode::MediaPause,
            MediaPlay => MudaCode::MediaPlay,
            MediaRecord => MudaCode::MediaRecord,
            MediaRewind => MudaCode::MediaRewind,
            MicrophoneMuteToggle => MudaCode::MicrophoneMuteToggle,
            PrivacyScreenToggle => MudaCode::PrivacyScreenToggle,
            SelectTask => MudaCode::SelectTask,
            ShowAllWindows => MudaCode::ShowAllWindows,
            ZoomToggle => MudaCode::ZoomToggle,
        }
    }
}

/// Serde-fähiger Enum für Modifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AcceleratorModifier {
    #[serde(rename = "alt")]
    ALT,
    #[serde(rename = "altgraph")]
    AltGraph,
    #[serde(rename = "scrolllock")]
    ScrollLock,
    #[serde(rename = "shift")]
    SHIFT,
    #[serde(rename = "super")]
    SUPER,
    #[serde(rename = "symbol")]
    SYMBOL,
    #[serde(rename = "symbollock")]
    SymbolLock,
    #[serde(rename = "fn")]
    FN,
    #[serde(rename = "fnlock")]
    FnLock,
    #[serde(rename = "numlock")]
    NumLock,
    #[serde(rename = "capslock")]
    CapsLock,
    #[serde(rename = "control")]
    CONTROL,
    #[serde(rename = "hyper")]
    HYPER,
    #[serde(rename = "meta")]
    META,
}

impl From<AcceleratorModifier> for MudaModifiers {
    fn from(modifier: AcceleratorModifier) -> Self {
        match modifier {
            AcceleratorModifier::ALT => MudaModifiers::ALT,
            AcceleratorModifier::AltGraph => MudaModifiers::ALT_GRAPH,
            AcceleratorModifier::ScrollLock => MudaModifiers::SCROLL_LOCK,
            AcceleratorModifier::SHIFT => MudaModifiers::SHIFT,
            AcceleratorModifier::SUPER => MudaModifiers::SUPER,
            AcceleratorModifier::SYMBOL => MudaModifiers::SYMBOL,
            AcceleratorModifier::SymbolLock => MudaModifiers::SYMBOL_LOCK,
            AcceleratorModifier::FN => MudaModifiers::FN,
            AcceleratorModifier::FnLock => MudaModifiers::FN_LOCK,
            AcceleratorModifier::NumLock => MudaModifiers::NUM_LOCK,
            AcceleratorModifier::CapsLock => MudaModifiers::CAPS_LOCK,
            AcceleratorModifier::CONTROL => MudaModifiers::CONTROL,
            AcceleratorModifier::HYPER => MudaModifiers::HYPER,
            AcceleratorModifier::META => MudaModifiers::META,
        }
    }
}

/// Serde-fähiger Enum für die nativen Icons.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeIconName {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "advanced")]
    Advanced,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "bookmarks")]
    Bookmarks,
    #[serde(rename = "caution")]
    Caution,
    #[serde(rename = "colorpanel")]
    ColorPanel,
    #[serde(rename = "columnview")]
    ColumnView,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "enterfullscreen")]
    EnterFullScreen,
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "exitfullscreen")]
    ExitFullScreen,
    #[serde(rename = "flowview")]
    FlowView,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "folderburnable")]
    FolderBurnable,
    #[serde(rename = "foldersmart")]
    FolderSmart,
    #[serde(rename = "followlinkfreestanding")]
    FollowLinkFreestanding,
    #[serde(rename = "fontpanel")]
    FontPanel,
    #[serde(rename = "goleft")]
    GoLeft,
    #[serde(rename = "goright")]
    GoRight,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "ichattheater")]
    IChatTheater,
    #[serde(rename = "iconview")]
    IconView,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "invaliddatafreestanding")]
    InvalidDataFreestanding,
    #[serde(rename = "leftfacingtriangle")]
    LeftFacingTriangle,
    #[serde(rename = "listview")]
    ListView,
    #[serde(rename = "locklocked")]
    LockLocked,
    #[serde(rename = "lockunlocked")]
    LockUnlocked,
    #[serde(rename = "menumixedstate")]
    MenuMixedState,
    #[serde(rename = "menuonstate")]
    MenuOnState,
    #[serde(rename = "mobileme")]
    MobileMe,
    #[serde(rename = "multipledocuments")]
    MultipleDocuments,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "preferencesgeneral")]
    PreferencesGeneral,
    #[serde(rename = "quicklook")]
    QuickLook,
    #[serde(rename = "refreshfreestanding")]
    RefreshFreestanding,
    #[serde(rename = "refresh")]
    Refresh,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "revealfreestanding")]
    RevealFreestanding,
    #[serde(rename = "rightfacingtriangle")]
    RightFacingTriangle,
    #[serde(rename = "share")]
    Share,
    #[serde(rename = "slideshow")]
    Slideshow,
    #[serde(rename = "smartbadge")]
    SmartBadge,
    #[serde(rename = "statusavailable")]
    StatusAvailable,
    #[serde(rename = "statusnone")]
    StatusNone,
    #[serde(rename = "statuspartiallyavailable")]
    StatusPartiallyAvailable,
    #[serde(rename = "statusunavailable")]
    StatusUnavailable,
    #[serde(rename = "stopprogressfreestanding")]
    StopProgressFreestanding,
    #[serde(rename = "stopprogress")]
    StopProgress,
    #[serde(rename = "trashempty")]
    TrashEmpty,
    #[serde(rename = "trashfull")]
    TrashFull,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "useraccounts")]
    UserAccounts,
    #[serde(rename = "usergroup")]
    UserGroup,
    #[serde(rename = "userguest")]
    UserGuest,
}

impl From<NativeIconName> for NativeIcon {
    fn from(icon_name: NativeIconName) -> Self {
        use NativeIconName::*;
        match icon_name {
            Add => NativeIcon::Add,
            Advanced => NativeIcon::Advanced,
            Bluetooth => NativeIcon::Bluetooth,
            Bookmarks => NativeIcon::Bookmarks,
            Caution => NativeIcon::Caution,
            ColorPanel => NativeIcon::ColorPanel,
            ColumnView => NativeIcon::ColumnView,
            Computer => NativeIcon::Computer,
            EnterFullScreen => NativeIcon::EnterFullScreen,
            Everyone => NativeIcon::Everyone,
            ExitFullScreen => NativeIcon::ExitFullScreen,
            FlowView => NativeIcon::FlowView,
            Folder => NativeIcon::Folder,
            FolderBurnable => NativeIcon::FolderBurnable,
            FolderSmart => NativeIcon::FolderSmart,
            FollowLinkFreestanding => NativeIcon::FollowLinkFreestanding,
            FontPanel => NativeIcon::FontPanel,
            GoLeft => NativeIcon::GoLeft,
            GoRight => NativeIcon::GoRight,
            Home => NativeIcon::Home,
            IChatTheater => NativeIcon::IChatTheater,
            IconView => NativeIcon::IconView,
            Info => NativeIcon::Info,
            InvalidDataFreestanding => NativeIcon::InvalidDataFreestanding,
            LeftFacingTriangle => NativeIcon::LeftFacingTriangle,
            ListView => NativeIcon::ListView,
            LockLocked => NativeIcon::LockLocked,
            LockUnlocked => NativeIcon::LockUnlocked,
            MenuMixedState => NativeIcon::MenuMixedState,
            MenuOnState => NativeIcon::MenuOnState,
            MobileMe => NativeIcon::MobileMe,
            MultipleDocuments => NativeIcon::MultipleDocuments,
            Network => NativeIcon::Network,
            Path => NativeIcon::Path,
            PreferencesGeneral => NativeIcon::PreferencesGeneral,
            QuickLook => NativeIcon::QuickLook,
            RefreshFreestanding => NativeIcon::RefreshFreestanding,
            Refresh => NativeIcon::Refresh,
            Remove => NativeIcon::Remove,
            RevealFreestanding => NativeIcon::RevealFreestanding,
            RightFacingTriangle => NativeIcon::RightFacingTriangle,
            Share => NativeIcon::Share,
            Slideshow => NativeIcon::Slideshow,
            SmartBadge => NativeIcon::SmartBadge,
            StatusAvailable => NativeIcon::StatusAvailable,
            StatusNone => NativeIcon::StatusNone,
            StatusPartiallyAvailable => NativeIcon::StatusPartiallyAvailable,
            StatusUnavailable => NativeIcon::StatusUnavailable,
            StopProgressFreestanding => NativeIcon::StopProgressFreestanding,
            StopProgress => NativeIcon::StopProgress,
            TrashEmpty => NativeIcon::TrashEmpty,
            TrashFull => NativeIcon::TrashFull,
            User => NativeIcon::User,
            UserAccounts => NativeIcon::UserAccounts,
            UserGroup => NativeIcon::UserGroup,
            UserGuest => NativeIcon::UserGuest,
        }
    }
}

#[allow(dead_code)]
pub fn create_hotkey_accelerator(
    modifier: Option<AcceleratorModifier>,
    key: AcceleratorCode,
) -> anyhow::Result<(u32, muda::accelerator::Accelerator)> {
    let modifiers = match modifier {
        Some(m) => m.into(),
        None => muda::accelerator::Modifiers::empty(),
    };

    let accelerator = muda::accelerator::Accelerator::new(Some(modifiers), key.into());
    let accelerator_id = accelerator.id();
    Ok((accelerator_id, accelerator))
}

#[allow(dead_code)]
pub fn create_muda_accelerator(
    modifier: Option<AcceleratorModifier>,
    key: AcceleratorCode,
) -> muda::accelerator::Accelerator {
    let modifiers = match modifier {
        Some(m) => m.into(),
        None => muda::accelerator::Modifiers::empty(),
    };

    muda::accelerator::Accelerator::new(Some(modifiers), key.into())
}
pub fn muda_menu_icon(icon_path: PathBuf) -> Option<muda::Icon> {
    let icon_object = match read(&icon_path) {
        Ok(bytes) => match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
            Ok(loaded) => {
                let image_buffer = loaded.to_rgba8();
                let (icon_width, icon_height) = image_buffer.dimensions();
                let icon_rgba = image_buffer.into_raw();

                match muda::Icon::from_rgba(icon_rgba, icon_width, icon_height) {
                    Ok(icon) => Some(icon),
                    Err(_) => {
                        println!("Failed to create icon from RGBA data.");
                        None
                    }
                }
            }
            Err(_) => {
                println!("Failed to load the image from the specified icon path.");
                None
            }
        },
        Err(_) => {
            println!("Failed to read the icon file from path: {:?}", icon_path);
            None
        }
    };

    icon_object
}

#[allow(dead_code)]
pub fn create_muda_about_metadata_menu_item(
    metadata: Option<crate::options::menu::AboutMetadata>,
) -> muda::AboutMetadata {
    let mut about_metadata = muda::AboutMetadata::default();

    if let Some(metadata) = metadata {
        about_metadata.name = metadata.name.clone();
        about_metadata.version = metadata.version.clone();
        about_metadata.website = metadata.website.clone();
        about_metadata.short_version = metadata.short_version.clone();
        about_metadata.authors = metadata.authors.clone();
        about_metadata.comments = metadata.comments.clone();
        about_metadata.copyright = metadata.copyright.clone();
        about_metadata.license = metadata.license.clone();
        about_metadata.website_label = metadata.website_label.clone();
        about_metadata.credits = metadata.credits.clone();

        // Set the icon if it exists and is provided in the metadata
        if let Some(icon) = metadata.icon.clone() {
            about_metadata.icon = muda_menu_icon(icon);
        }
    }

    about_metadata
}
#[allow(dead_code)]
pub fn system_tray_icon(icon_path: PathBuf) -> tray_icon::Icon {
    // Icon-Datei lesen
    let bytes = read(&icon_path).unwrap_or_else(|_| panic!("Failed to read the icon file from path: {:?}", icon_path));

    // Bild laden
    let loaded = image::load_from_memory_with_format(&bytes, ImageFormat::Png)
        .unwrap_or_else(|_| panic!("Failed to load the image from the specified icon path."));

    // In RGBA konvertieren
    let image_buffer = loaded.to_rgba8();
    let (icon_width, icon_height) = image_buffer.dimensions();
    let icon_rgba = image_buffer.into_raw();

    // Icon erstellen
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .unwrap_or_else(|_| panic!("Failed to create icon from RGBA data."))
}

#[allow(dead_code)]
pub fn create_predefined_menu_item(
    item: PredefinedMenuItem,
) -> (muda::MenuId, muda::PredefinedMenuItem, Option<String>) {
    let predefind_metadata = create_muda_about_metadata_menu_item(item.metadata);
    let predefined_item = match item.item_type.as_str() {
        "separator" => muda::PredefinedMenuItem::separator(),
        "about" => muda::PredefinedMenuItem::about(None, Some(predefind_metadata)),
        "close_window" => muda::PredefinedMenuItem::close_window(item.text.as_deref()),
        "copy" => muda::PredefinedMenuItem::copy(item.text.as_deref()),
        "fullscreen" => muda::PredefinedMenuItem::fullscreen(item.text.as_deref()),
        "cut" => muda::PredefinedMenuItem::cut(item.text.as_deref()),
        "hide" => muda::PredefinedMenuItem::hide(item.text.as_deref()),
        "hide_others" => muda::PredefinedMenuItem::hide_others(item.text.as_deref()),
        "maximize" => muda::PredefinedMenuItem::maximize(item.text.as_deref()),
        "minimize" => muda::PredefinedMenuItem::minimize(item.text.as_deref()),
        "paste" => muda::PredefinedMenuItem::paste(item.text.as_deref()),
        "bring_all_to_front" => muda::PredefinedMenuItem::bring_all_to_front(item.text.as_deref()),
        "quit" => muda::PredefinedMenuItem::quit(item.text.as_deref()),
        "redo" => muda::PredefinedMenuItem::redo(item.text.as_deref()),
        "select_all" => muda::PredefinedMenuItem::select_all(item.text.as_deref()),
        "show_all" => muda::PredefinedMenuItem::show_all(item.text.as_deref()),
        "undo" => muda::PredefinedMenuItem::undo(item.text.as_deref()),
        _ => {
            println!("Unsupported menu item type: {}", item.item_type);
            panic!("Unsupported menu item type")
        }
    };
    let py_function = item.command_id.clone();
    let predefined_item_id = predefined_item.id().clone();
    (predefined_item_id, predefined_item, py_function)
}

pub fn create_check_menuitem(item: CheckMenuItem) -> (muda::MenuId, muda::CheckMenuItem, Option<String>) {
    let accelerator = create_muda_accelerator(Some(item.modifier.clone()), item.key.clone());
    let check_item = muda::CheckMenuItem::new(item.text.clone(), item.enabled, item.checked, Some(accelerator));
    let py_function = item.command_id.clone();

    let check_item_id = check_item.id().clone();
    (check_item_id, check_item, py_function)
}

pub fn create_icon_menuitem(item: IconMenuItem) -> (muda::MenuId, muda::IconMenuItem, Option<String>) {
    let menu_icon = muda_menu_icon(item.icon_path.clone());
    let accelerator = create_muda_accelerator(Some(item.modifier.clone()), item.key.clone());
    let icon_item = muda::IconMenuItem::new(item.text.clone(), item.enabled, menu_icon, Some(accelerator));
    let py_function = item.command_id.clone();
    let icon_item_id = icon_item.id().clone();
    (icon_item_id, icon_item, py_function)
}

pub fn create_menuitem(item: MenuItem) -> (muda::MenuId, muda::MenuItem, Option<String>) {
    let accelerator = create_muda_accelerator(Some(item.modifier.clone()), item.key.clone());
    let normal_item = muda::MenuItem::new(item.text.clone(), item.enabled, Some(accelerator));
    let py_function = item.command_id.clone();
    let normal_item_id = normal_item.id().clone();
    (normal_item_id, normal_item, py_function)
}

/// Erzeugt ein Submenü und fügt es in die übergebene Map ein.
pub fn create_submenu(
    item: Submenu,
    menu_api: &mut HashMap<muda::MenuId, (muda::MenuItemKind, Option<String>)>,
) -> muda::Submenu {
    let submenu = muda::Submenu::new(item.text.clone(), item.enabled);

    if let Some(menu_items) = item.menu_items {
        for menu_item in menu_items {
            let (menu_id, menu_item, py_command) = create_menuitem(menu_item.clone());
            submenu.append(&menu_item).ok();
            menu_api.insert(menu_id, (muda::MenuItemKind::MenuItem(menu_item), py_command));
        }
    }

    if let Some(icon_menus) = item.icon_menu {
        for icon_menu in icon_menus {
            let (menu_id, menu_item, py_command) = create_icon_menuitem(icon_menu.clone());
            submenu.append(&menu_item).ok();
            menu_api.insert(menu_id, (muda::MenuItemKind::Icon(menu_item), py_command));
        }
    }

    if let Some(check_menus) = item.check_menu {
        for check_menu in check_menus {
            let (menu_id, menu_item, py_command) = create_check_menuitem(check_menu);
            submenu.append(&menu_item).ok();
            menu_api.insert(menu_id, (muda::MenuItemKind::Check(menu_item), py_command));
        }
    }

    if let Some(predefined_menu) = &item.predefined_menu {
        for predefined_menu in predefined_menu {
            let (menu_id, menu_item, py_command) = create_predefined_menu_item(predefined_menu.clone());
            submenu.append(&menu_item).ok();
            menu_api.insert(menu_id, (muda::MenuItemKind::Predefined(menu_item), py_command));
        }
    }

    submenu
}

pub fn init_sys_tray(tra_options: SystemTray, window_menu: muda::Menu) -> anyhow::Result<tray_icon::TrayIcon> {
    let mut builder = tray_icon::TrayIconBuilder::new();
    if let Some(icon_path) = &tra_options.icon {
        let icon = crate::hylper::system_tray_icon(icon_path.into());
        builder = builder.with_icon(icon);
    }
    #[cfg(not(target_os = "windows"))]
    if let Some(title) = &tra_options.title {
        builder = builder.with_title(title);
    }
    #[cfg(target_os = "macos")]
    if let Some(is_template) = &tra_options.is_template {
        builder = builder.with_icon_as_template(*is_template);
    }
    #[cfg(not(target_os = "linux"))]
    if let Some(is_template) = &tra_options.is_template {
        builder = builder.with_menu_on_left_click(*is_template);
    }
    #[cfg(target_os = "linux")]
    if let Some(temp_dir_path) = &tra_options.temp_dir_path {
        builder = builder.with_temp_dir_path(temp_dir_path);
    }
    #[cfg(not(target_os = "linux"))]
    if let Some(tooltip) = &tra_options.tooltip {
        builder = builder.with_tooltip(tooltip);
    }
    let tray = builder.with_menu(Box::new(window_menu)).build()?;
    Ok(tray)
}
