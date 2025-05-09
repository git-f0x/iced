// Borrowed from winit
use winit::keyboard::{KeyCode, NativeKeyCode, PhysicalKey};
/// Map the raw X11-style keycode to the `KeyCode` enum.
///
/// X11-style keycodes are offset by 8 from the keycodes the Linux kernel uses.
pub fn raw_keycode_to_physicalkey(keycode: u32) -> PhysicalKey {
    scancode_to_physicalkey(keycode.saturating_sub(8))
}

/// Map the linux scancode to Keycode.
///
/// Both X11 and Wayland use keys with `+ 8` offset to linux scancode.
pub fn scancode_to_physicalkey(scancode: u32) -> PhysicalKey {
    // The keycode values are taken from linux/include/uapi/linux/input-event-codes.h, as
    // libxkbcommon's documentation seems to suggest that the keycode values we're interested in
    // are defined by the Linux kernel. If Winit programs end up being run on other Unix-likes,
    // I can only hope they agree on what the keycodes mean.
    //
    // Some of the keycodes are likely superfluous for our purposes, and some are ones which are
    // difficult to test the correctness of, or discover the purpose of. Because of this, they've
    // either been commented out here, or not included at all.
    PhysicalKey::Code(match scancode {
        0 => return PhysicalKey::Unidentified(NativeKeyCode::Xkb(0)),
        1 => KeyCode::Escape,
        2 => KeyCode::Digit1,
        3 => KeyCode::Digit2,
        4 => KeyCode::Digit3,
        5 => KeyCode::Digit4,
        6 => KeyCode::Digit5,
        7 => KeyCode::Digit6,
        8 => KeyCode::Digit7,
        9 => KeyCode::Digit8,
        10 => KeyCode::Digit9,
        11 => KeyCode::Digit0,
        12 => KeyCode::Minus,
        13 => KeyCode::Equal,
        14 => KeyCode::Backspace,
        15 => KeyCode::Tab,
        16 => KeyCode::KeyQ,
        17 => KeyCode::KeyW,
        18 => KeyCode::KeyE,
        19 => KeyCode::KeyR,
        20 => KeyCode::KeyT,
        21 => KeyCode::KeyY,
        22 => KeyCode::KeyU,
        23 => KeyCode::KeyI,
        24 => KeyCode::KeyO,
        25 => KeyCode::KeyP,
        26 => KeyCode::BracketLeft,
        27 => KeyCode::BracketRight,
        28 => KeyCode::Enter,
        29 => KeyCode::ControlLeft,
        30 => KeyCode::KeyA,
        31 => KeyCode::KeyS,
        32 => KeyCode::KeyD,
        33 => KeyCode::KeyF,
        34 => KeyCode::KeyG,
        35 => KeyCode::KeyH,
        36 => KeyCode::KeyJ,
        37 => KeyCode::KeyK,
        38 => KeyCode::KeyL,
        39 => KeyCode::Semicolon,
        40 => KeyCode::Quote,
        41 => KeyCode::Backquote,
        42 => KeyCode::ShiftLeft,
        43 => KeyCode::Backslash,
        44 => KeyCode::KeyZ,
        45 => KeyCode::KeyX,
        46 => KeyCode::KeyC,
        47 => KeyCode::KeyV,
        48 => KeyCode::KeyB,
        49 => KeyCode::KeyN,
        50 => KeyCode::KeyM,
        51 => KeyCode::Comma,
        52 => KeyCode::Period,
        53 => KeyCode::Slash,
        54 => KeyCode::ShiftRight,
        55 => KeyCode::NumpadMultiply,
        56 => KeyCode::AltLeft,
        57 => KeyCode::Space,
        58 => KeyCode::CapsLock,
        59 => KeyCode::F1,
        60 => KeyCode::F2,
        61 => KeyCode::F3,
        62 => KeyCode::F4,
        63 => KeyCode::F5,
        64 => KeyCode::F6,
        65 => KeyCode::F7,
        66 => KeyCode::F8,
        67 => KeyCode::F9,
        68 => KeyCode::F10,
        69 => KeyCode::NumLock,
        70 => KeyCode::ScrollLock,
        71 => KeyCode::Numpad7,
        72 => KeyCode::Numpad8,
        73 => KeyCode::Numpad9,
        74 => KeyCode::NumpadSubtract,
        75 => KeyCode::Numpad4,
        76 => KeyCode::Numpad5,
        77 => KeyCode::Numpad6,
        78 => KeyCode::NumpadAdd,
        79 => KeyCode::Numpad1,
        80 => KeyCode::Numpad2,
        81 => KeyCode::Numpad3,
        82 => KeyCode::Numpad0,
        83 => KeyCode::NumpadDecimal,
        85 => KeyCode::Lang5,
        86 => KeyCode::IntlBackslash,
        87 => KeyCode::F11,
        88 => KeyCode::F12,
        89 => KeyCode::IntlRo,
        90 => KeyCode::Lang3,
        91 => KeyCode::Lang4,
        92 => KeyCode::Convert,
        93 => KeyCode::KanaMode,
        94 => KeyCode::NonConvert,
        // 95 => KeyCode::KPJPCOMMA,
        96 => KeyCode::NumpadEnter,
        97 => KeyCode::ControlRight,
        98 => KeyCode::NumpadDivide,
        99 => KeyCode::PrintScreen,
        100 => KeyCode::AltRight,
        // 101 => KeyCode::LINEFEED,
        102 => KeyCode::Home,
        103 => KeyCode::ArrowUp,
        104 => KeyCode::PageUp,
        105 => KeyCode::ArrowLeft,
        106 => KeyCode::ArrowRight,
        107 => KeyCode::End,
        108 => KeyCode::ArrowDown,
        109 => KeyCode::PageDown,
        110 => KeyCode::Insert,
        111 => KeyCode::Delete,
        // 112 => KeyCode::MACRO,
        113 => KeyCode::AudioVolumeMute,
        114 => KeyCode::AudioVolumeDown,
        115 => KeyCode::AudioVolumeUp,
        // 116 => KeyCode::POWER,
        117 => KeyCode::NumpadEqual,
        // 118 => KeyCode::KPPLUSMINUS,
        119 => KeyCode::Pause,
        // 120 => KeyCode::SCALE,
        121 => KeyCode::NumpadComma,
        122 => KeyCode::Lang1,
        123 => KeyCode::Lang2,
        124 => KeyCode::IntlYen,
        125 => KeyCode::SuperLeft,
        126 => KeyCode::SuperRight,
        127 => KeyCode::ContextMenu,
        // 128 => KeyCode::STOP,
        // 129 => KeyCode::AGAIN,
        // 130 => KeyCode::PROPS,
        // 131 => KeyCode::UNDO,
        // 132 => KeyCode::FRONT,
        // 133 => KeyCode::COPY,
        // 134 => KeyCode::OPEN,
        // 135 => KeyCode::PASTE,
        // 136 => KeyCode::FIND,
        // 137 => KeyCode::CUT,
        // 138 => KeyCode::HELP,
        // 139 => KeyCode::MENU,
        // 140 => KeyCode::CALC,
        // 141 => KeyCode::SETUP,
        // 142 => KeyCode::SLEEP,
        // 143 => KeyCode::WAKEUP,
        // 144 => KeyCode::FILE,
        // 145 => KeyCode::SENDFILE,
        // 146 => KeyCode::DELETEFILE,
        // 147 => KeyCode::XFER,
        // 148 => KeyCode::PROG1,
        // 149 => KeyCode::PROG2,
        // 150 => KeyCode::WWW,
        // 151 => KeyCode::MSDOS,
        // 152 => KeyCode::COFFEE,
        // 153 => KeyCode::ROTATE_DISPLAY,
        // 154 => KeyCode::CYCLEWINDOWS,
        // 155 => KeyCode::MAIL,
        // 156 => KeyCode::BOOKMARKS,
        // 157 => KeyCode::COMPUTER,
        // 158 => KeyCode::BACK,
        // 159 => KeyCode::FORWARD,
        // 160 => KeyCode::CLOSECD,
        // 161 => KeyCode::EJECTCD,
        // 162 => KeyCode::EJECTCLOSECD,
        163 => KeyCode::MediaTrackNext,
        164 => KeyCode::MediaPlayPause,
        165 => KeyCode::MediaTrackPrevious,
        166 => KeyCode::MediaStop,
        // 167 => KeyCode::RECORD,
        // 168 => KeyCode::REWIND,
        // 169 => KeyCode::PHONE,
        // 170 => KeyCode::ISO,
        // 171 => KeyCode::CONFIG,
        // 172 => KeyCode::HOMEPAGE,
        // 173 => KeyCode::REFRESH,
        // 174 => KeyCode::EXIT,
        // 175 => KeyCode::MOVE,
        // 176 => KeyCode::EDIT,
        // 177 => KeyCode::SCROLLUP,
        // 178 => KeyCode::SCROLLDOWN,
        // 179 => KeyCode::KPLEFTPAREN,
        // 180 => KeyCode::KPRIGHTPAREN,
        // 181 => KeyCode::NEW,
        // 182 => KeyCode::REDO,
        183 => KeyCode::F13,
        184 => KeyCode::F14,
        185 => KeyCode::F15,
        186 => KeyCode::F16,
        187 => KeyCode::F17,
        188 => KeyCode::F18,
        189 => KeyCode::F19,
        190 => KeyCode::F20,
        191 => KeyCode::F21,
        192 => KeyCode::F22,
        193 => KeyCode::F23,
        194 => KeyCode::F24,
        // 200 => KeyCode::PLAYCD,
        // 201 => KeyCode::PAUSECD,
        // 202 => KeyCode::PROG3,
        // 203 => KeyCode::PROG4,
        // 204 => KeyCode::DASHBOARD,
        // 205 => KeyCode::SUSPEND,
        // 206 => KeyCode::CLOSE,
        // 207 => KeyCode::PLAY,
        // 208 => KeyCode::FASTFORWARD,
        // 209 => KeyCode::BASSBOOST,
        // 210 => KeyCode::PRINT,
        // 211 => KeyCode::HP,
        // 212 => KeyCode::CAMERA,
        // 213 => KeyCode::SOUND,
        // 214 => KeyCode::QUESTION,
        // 215 => KeyCode::EMAIL,
        // 216 => KeyCode::CHAT,
        // 217 => KeyCode::SEARCH,
        // 218 => KeyCode::CONNECT,
        // 219 => KeyCode::FINANCE,
        // 220 => KeyCode::SPORT,
        // 221 => KeyCode::SHOP,
        // 222 => KeyCode::ALTERASE,
        // 223 => KeyCode::CANCEL,
        // 224 => KeyCode::BRIGHTNESSDOW,
        // 225 => KeyCode::BRIGHTNESSU,
        // 226 => KeyCode::MEDIA,
        // 227 => KeyCode::SWITCHVIDEOMODE,
        // 228 => KeyCode::KBDILLUMTOGGLE,
        // 229 => KeyCode::KBDILLUMDOWN,
        // 230 => KeyCode::KBDILLUMUP,
        // 231 => KeyCode::SEND,
        // 232 => KeyCode::REPLY,
        // 233 => KeyCode::FORWARDMAIL,
        // 234 => KeyCode::SAVE,
        // 235 => KeyCode::DOCUMENTS,
        // 236 => KeyCode::BATTERY,
        // 237 => KeyCode::BLUETOOTH,
        // 238 => KeyCode::WLAN,
        // 239 => KeyCode::UWB,
        240 => return PhysicalKey::Unidentified(NativeKeyCode::Unidentified),
        // 241 => KeyCode::VIDEO_NEXT,
        // 242 => KeyCode::VIDEO_PREV,
        // 243 => KeyCode::BRIGHTNESS_CYCLE,
        // 244 => KeyCode::BRIGHTNESS_AUTO,
        // 245 => KeyCode::DISPLAY_OFF,
        // 246 => KeyCode::WWAN,
        // 247 => KeyCode::RFKILL,
        // 248 => KeyCode::KEY_MICMUTE,
        _ => return PhysicalKey::Unidentified(NativeKeyCode::Xkb(scancode)),
    })
}

pub fn physicalkey_to_scancode(key: PhysicalKey) -> Option<u32> {
    let code = match key {
        PhysicalKey::Code(code) => code,
        PhysicalKey::Unidentified(code) => {
            return match code {
                NativeKeyCode::Unidentified => Some(240),
                NativeKeyCode::Xkb(raw) => Some(raw),
                _ => None,
            };
        }
    };

    match code {
        KeyCode::Escape => Some(1),
        KeyCode::Digit1 => Some(2),
        KeyCode::Digit2 => Some(3),
        KeyCode::Digit3 => Some(4),
        KeyCode::Digit4 => Some(5),
        KeyCode::Digit5 => Some(6),
        KeyCode::Digit6 => Some(7),
        KeyCode::Digit7 => Some(8),
        KeyCode::Digit8 => Some(9),
        KeyCode::Digit9 => Some(10),
        KeyCode::Digit0 => Some(11),
        KeyCode::Minus => Some(12),
        KeyCode::Equal => Some(13),
        KeyCode::Backspace => Some(14),
        KeyCode::Tab => Some(15),
        KeyCode::KeyQ => Some(16),
        KeyCode::KeyW => Some(17),
        KeyCode::KeyE => Some(18),
        KeyCode::KeyR => Some(19),
        KeyCode::KeyT => Some(20),
        KeyCode::KeyY => Some(21),
        KeyCode::KeyU => Some(22),
        KeyCode::KeyI => Some(23),
        KeyCode::KeyO => Some(24),
        KeyCode::KeyP => Some(25),
        KeyCode::BracketLeft => Some(26),
        KeyCode::BracketRight => Some(27),
        KeyCode::Enter => Some(28),
        KeyCode::ControlLeft => Some(29),
        KeyCode::KeyA => Some(30),
        KeyCode::KeyS => Some(31),
        KeyCode::KeyD => Some(32),
        KeyCode::KeyF => Some(33),
        KeyCode::KeyG => Some(34),
        KeyCode::KeyH => Some(35),
        KeyCode::KeyJ => Some(36),
        KeyCode::KeyK => Some(37),
        KeyCode::KeyL => Some(38),
        KeyCode::Semicolon => Some(39),
        KeyCode::Quote => Some(40),
        KeyCode::Backquote => Some(41),
        KeyCode::ShiftLeft => Some(42),
        KeyCode::Backslash => Some(43),
        KeyCode::KeyZ => Some(44),
        KeyCode::KeyX => Some(45),
        KeyCode::KeyC => Some(46),
        KeyCode::KeyV => Some(47),
        KeyCode::KeyB => Some(48),
        KeyCode::KeyN => Some(49),
        KeyCode::KeyM => Some(50),
        KeyCode::Comma => Some(51),
        KeyCode::Period => Some(52),
        KeyCode::Slash => Some(53),
        KeyCode::ShiftRight => Some(54),
        KeyCode::NumpadMultiply => Some(55),
        KeyCode::AltLeft => Some(56),
        KeyCode::Space => Some(57),
        KeyCode::CapsLock => Some(58),
        KeyCode::F1 => Some(59),
        KeyCode::F2 => Some(60),
        KeyCode::F3 => Some(61),
        KeyCode::F4 => Some(62),
        KeyCode::F5 => Some(63),
        KeyCode::F6 => Some(64),
        KeyCode::F7 => Some(65),
        KeyCode::F8 => Some(66),
        KeyCode::F9 => Some(67),
        KeyCode::F10 => Some(68),
        KeyCode::NumLock => Some(69),
        KeyCode::ScrollLock => Some(70),
        KeyCode::Numpad7 => Some(71),
        KeyCode::Numpad8 => Some(72),
        KeyCode::Numpad9 => Some(73),
        KeyCode::NumpadSubtract => Some(74),
        KeyCode::Numpad4 => Some(75),
        KeyCode::Numpad5 => Some(76),
        KeyCode::Numpad6 => Some(77),
        KeyCode::NumpadAdd => Some(78),
        KeyCode::Numpad1 => Some(79),
        KeyCode::Numpad2 => Some(80),
        KeyCode::Numpad3 => Some(81),
        KeyCode::Numpad0 => Some(82),
        KeyCode::NumpadDecimal => Some(83),
        KeyCode::Lang5 => Some(85),
        KeyCode::IntlBackslash => Some(86),
        KeyCode::F11 => Some(87),
        KeyCode::F12 => Some(88),
        KeyCode::IntlRo => Some(89),
        KeyCode::Lang3 => Some(90),
        KeyCode::Lang4 => Some(91),
        KeyCode::Convert => Some(92),
        KeyCode::KanaMode => Some(93),
        KeyCode::NonConvert => Some(94),
        KeyCode::NumpadEnter => Some(96),
        KeyCode::ControlRight => Some(97),
        KeyCode::NumpadDivide => Some(98),
        KeyCode::PrintScreen => Some(99),
        KeyCode::AltRight => Some(100),
        KeyCode::Home => Some(102),
        KeyCode::ArrowUp => Some(103),
        KeyCode::PageUp => Some(104),
        KeyCode::ArrowLeft => Some(105),
        KeyCode::ArrowRight => Some(106),
        KeyCode::End => Some(107),
        KeyCode::ArrowDown => Some(108),
        KeyCode::PageDown => Some(109),
        KeyCode::Insert => Some(110),
        KeyCode::Delete => Some(111),
        KeyCode::AudioVolumeMute => Some(113),
        KeyCode::AudioVolumeDown => Some(114),
        KeyCode::AudioVolumeUp => Some(115),
        KeyCode::NumpadEqual => Some(117),
        KeyCode::Pause => Some(119),
        KeyCode::NumpadComma => Some(121),
        KeyCode::Lang1 => Some(122),
        KeyCode::Lang2 => Some(123),
        KeyCode::IntlYen => Some(124),
        KeyCode::SuperLeft => Some(125),
        KeyCode::SuperRight => Some(126),
        KeyCode::ContextMenu => Some(127),
        KeyCode::MediaTrackNext => Some(163),
        KeyCode::MediaPlayPause => Some(164),
        KeyCode::MediaTrackPrevious => Some(165),
        KeyCode::MediaStop => Some(166),
        KeyCode::F13 => Some(183),
        KeyCode::F14 => Some(184),
        KeyCode::F15 => Some(185),
        KeyCode::F16 => Some(186),
        KeyCode::F17 => Some(187),
        KeyCode::F18 => Some(188),
        KeyCode::F19 => Some(189),
        KeyCode::F20 => Some(190),
        KeyCode::F21 => Some(191),
        KeyCode::F22 => Some(192),
        KeyCode::F23 => Some(193),
        KeyCode::F24 => Some(194),
        _ => None,
    }
}

pub fn keysym_to_key(keysym: u32) -> Key {
    use xkbcommon_dl::keysyms;
    Key::Named(match keysym {
        // TTY function keys
        keysyms::BackSpace => Named::Backspace,
        keysyms::Tab => Named::Tab,
        // keysyms::Linefeed => Named::Linefeed,
        keysyms::Clear => Named::Clear,
        keysyms::Return => Named::Enter,
        keysyms::Pause => Named::Pause,
        keysyms::Scroll_Lock => Named::ScrollLock,
        keysyms::Sys_Req => Named::PrintScreen,
        keysyms::Escape => Named::Escape,
        keysyms::Delete => Named::Delete,

        // IME keys
        keysyms::Multi_key => Named::Compose,
        keysyms::Codeinput => Named::CodeInput,
        keysyms::SingleCandidate => Named::SingleCandidate,
        keysyms::MultipleCandidate => Named::AllCandidates,
        keysyms::PreviousCandidate => Named::PreviousCandidate,

        // Japanese key
        keysyms::Kanji => Named::KanjiMode,
        keysyms::Muhenkan => Named::NonConvert,
        keysyms::Henkan_Mode => Named::Convert,
        keysyms::Romaji => Named::Romaji,
        keysyms::Hiragana => Named::Hiragana,
        keysyms::Hiragana_Katakana => Named::HiraganaKatakana,
        keysyms::Zenkaku => Named::Zenkaku,
        keysyms::Hankaku => Named::Hankaku,
        keysyms::Zenkaku_Hankaku => Named::ZenkakuHankaku,
        // keysyms::Touroku => Named::Touroku,
        // keysyms::Massyo => Named::Massyo,
        keysyms::Kana_Lock => Named::KanaMode,
        keysyms::Kana_Shift => Named::KanaMode,
        keysyms::Eisu_Shift => Named::Alphanumeric,
        keysyms::Eisu_toggle => Named::Alphanumeric,
        // NOTE: The next three items are aliases for values we've already mapped.
        // keysyms::Kanji_Bangou => Named::CodeInput,
        // keysyms::Zen_Koho => Named::AllCandidates,
        // keysyms::Mae_Koho => Named::PreviousCandidate,

        // Cursor control & motion
        keysyms::Home => Named::Home,
        keysyms::Left => Named::ArrowLeft,
        keysyms::Up => Named::ArrowUp,
        keysyms::Right => Named::ArrowRight,
        keysyms::Down => Named::ArrowDown,
        // keysyms::Prior => Named::PageUp,
        keysyms::Page_Up => Named::PageUp,
        // keysyms::Next => Named::PageDown,
        keysyms::Page_Down => Named::PageDown,
        keysyms::End => Named::End,
        // keysyms::Begin => Named::Begin,

        // Misc. functions
        keysyms::Select => Named::Select,
        keysyms::Print => Named::PrintScreen,
        keysyms::Execute => Named::Execute,
        keysyms::Insert => Named::Insert,
        keysyms::Undo => Named::Undo,
        keysyms::Redo => Named::Redo,
        keysyms::Menu => Named::ContextMenu,
        keysyms::Find => Named::Find,
        keysyms::Cancel => Named::Cancel,
        keysyms::Help => Named::Help,
        keysyms::Break => Named::Pause,
        keysyms::Mode_switch => Named::ModeChange,
        // keysyms::script_switch => Named::ModeChange,
        keysyms::Num_Lock => Named::NumLock,

        // Keypad keys
        // keysyms::KP_Space => return Key::Character(" "),
        keysyms::KP_Tab => Named::Tab,
        keysyms::KP_Enter => Named::Enter,
        keysyms::KP_F1 => Named::F1,
        keysyms::KP_F2 => Named::F2,
        keysyms::KP_F3 => Named::F3,
        keysyms::KP_F4 => Named::F4,
        keysyms::KP_Home => Named::Home,
        keysyms::KP_Left => Named::ArrowLeft,
        keysyms::KP_Up => Named::ArrowUp,
        keysyms::KP_Right => Named::ArrowRight,
        keysyms::KP_Down => Named::ArrowDown,
        // keysyms::KP_Prior => Named::PageUp,
        keysyms::KP_Page_Up => Named::PageUp,
        // keysyms::KP_Next => Named::PageDown,
        keysyms::KP_Page_Down => Named::PageDown,
        keysyms::KP_End => Named::End,
        // This is the key labeled "5" on the numpad when NumLock is off.
        // keysyms::KP_Begin => Named::Begin,
        keysyms::KP_Insert => Named::Insert,
        keysyms::KP_Delete => Named::Delete,
        // keysyms::KP_Equal => Named::Equal,
        // keysyms::KP_Multiply => Named::Multiply,
        // keysyms::KP_Add => Named::Add,
        // keysyms::KP_Separator => Named::Separator,
        // keysyms::KP_Subtract => Named::Subtract,
        // keysyms::KP_Decimal => Named::Decimal,
        // keysyms::KP_Divide => Named::Divide,

        // keysyms::KP_0 => return Key::Character("0"),
        // keysyms::KP_1 => return Key::Character("1"),
        // keysyms::KP_2 => return Key::Character("2"),
        // keysyms::KP_3 => return Key::Character("3"),
        // keysyms::KP_4 => return Key::Character("4"),
        // keysyms::KP_5 => return Key::Character("5"),
        // keysyms::KP_6 => return Key::Character("6"),
        // keysyms::KP_7 => return Key::Character("7"),
        // keysyms::KP_8 => return Key::Character("8"),
        // keysyms::KP_9 => return Key::Character("9"),

        // Function keys
        keysyms::F1 => Named::F1,
        keysyms::F2 => Named::F2,
        keysyms::F3 => Named::F3,
        keysyms::F4 => Named::F4,
        keysyms::F5 => Named::F5,
        keysyms::F6 => Named::F6,
        keysyms::F7 => Named::F7,
        keysyms::F8 => Named::F8,
        keysyms::F9 => Named::F9,
        keysyms::F10 => Named::F10,
        keysyms::F11 => Named::F11,
        keysyms::F12 => Named::F12,
        keysyms::F13 => Named::F13,
        keysyms::F14 => Named::F14,
        keysyms::F15 => Named::F15,
        keysyms::F16 => Named::F16,
        keysyms::F17 => Named::F17,
        keysyms::F18 => Named::F18,
        keysyms::F19 => Named::F19,
        keysyms::F20 => Named::F20,
        keysyms::F21 => Named::F21,
        keysyms::F22 => Named::F22,
        keysyms::F23 => Named::F23,
        keysyms::F24 => Named::F24,
        keysyms::F25 => Named::F25,
        keysyms::F26 => Named::F26,
        keysyms::F27 => Named::F27,
        keysyms::F28 => Named::F28,
        keysyms::F29 => Named::F29,
        keysyms::F30 => Named::F30,
        keysyms::F31 => Named::F31,
        keysyms::F32 => Named::F32,
        keysyms::F33 => Named::F33,
        keysyms::F34 => Named::F34,
        keysyms::F35 => Named::F35,

        // Modifiers
        keysyms::Shift_L => Named::Shift,
        keysyms::Shift_R => Named::Shift,
        keysyms::Control_L => Named::Control,
        keysyms::Control_R => Named::Control,
        keysyms::Caps_Lock => Named::CapsLock,
        // keysyms::Shift_Lock => Named::ShiftLock,

        // keysyms::Meta_L => Named::Meta,
        // keysyms::Meta_R => Named::Meta,
        keysyms::Alt_L => Named::Alt,
        keysyms::Alt_R => Named::Alt,
        keysyms::Super_L => Named::Super,
        keysyms::Super_R => Named::Super,
        keysyms::Hyper_L => Named::Hyper,
        keysyms::Hyper_R => Named::Hyper,

        // XKB function and modifier keys
        // keysyms::ISO_Lock => Named::IsoLock,
        // keysyms::ISO_Level2_Latch => Named::IsoLevel2Latch,
        keysyms::ISO_Level3_Shift => Named::AltGraph,
        keysyms::ISO_Level3_Latch => Named::AltGraph,
        keysyms::ISO_Level3_Lock => Named::AltGraph,
        // keysyms::ISO_Level5_Shift => Named::IsoLevel5Shift,
        // keysyms::ISO_Level5_Latch => Named::IsoLevel5Latch,
        // keysyms::ISO_Level5_Lock => Named::IsoLevel5Lock,
        // keysyms::ISO_Group_Shift => Named::IsoGroupShift,
        // keysyms::ISO_Group_Latch => Named::IsoGroupLatch,
        // keysyms::ISO_Group_Lock => Named::IsoGroupLock,
        keysyms::ISO_Next_Group => Named::GroupNext,
        // keysyms::ISO_Next_Group_Lock => Named::GroupNextLock,
        keysyms::ISO_Prev_Group => Named::GroupPrevious,
        // keysyms::ISO_Prev_Group_Lock => Named::GroupPreviousLock,
        keysyms::ISO_First_Group => Named::GroupFirst,
        // keysyms::ISO_First_Group_Lock => Named::GroupFirstLock,
        keysyms::ISO_Last_Group => Named::GroupLast,
        // keysyms::ISO_Last_Group_Lock => Named::GroupLastLock,
        //
        keysyms::ISO_Left_Tab => Named::Tab,
        // keysyms::ISO_Move_Line_Up => Named::IsoMoveLineUp,
        // keysyms::ISO_Move_Line_Down => Named::IsoMoveLineDown,
        // keysyms::ISO_Partial_Line_Up => Named::IsoPartialLineUp,
        // keysyms::ISO_Partial_Line_Down => Named::IsoPartialLineDown,
        // keysyms::ISO_Partial_Space_Left => Named::IsoPartialSpaceLeft,
        // keysyms::ISO_Partial_Space_Right => Named::IsoPartialSpaceRight,
        // keysyms::ISO_Set_Margin_Left => Named::IsoSetMarginLeft,
        // keysyms::ISO_Set_Margin_Right => Named::IsoSetMarginRight,
        // keysyms::ISO_Release_Margin_Left => Named::IsoReleaseMarginLeft,
        // keysyms::ISO_Release_Margin_Right => Named::IsoReleaseMarginRight,
        // keysyms::ISO_Release_Both_Margins => Named::IsoReleaseBothMargins,
        // keysyms::ISO_Fast_Cursor_Left => Named::IsoFastCursorLeft,
        // keysyms::ISO_Fast_Cursor_Right => Named::IsoFastCursorRight,
        // keysyms::ISO_Fast_Cursor_Up => Named::IsoFastCursorUp,
        // keysyms::ISO_Fast_Cursor_Down => Named::IsoFastCursorDown,
        // keysyms::ISO_Continuous_Underline => Named::IsoContinuousUnderline,
        // keysyms::ISO_Discontinuous_Underline => Named::IsoDiscontinuousUnderline,
        // keysyms::ISO_Emphasize => Named::IsoEmphasize,
        // keysyms::ISO_Center_Object => Named::IsoCenterObject,
        keysyms::ISO_Enter => Named::Enter,

        // dead_grave..dead_currency

        // dead_lowline..dead_longsolidusoverlay

        // dead_a..dead_capital_schwa

        // dead_greek

        // First_Virtual_Screen..Terminate_Server

        // AccessX_Enable..AudibleBell_Enable

        // Pointer_Left..Pointer_Drag5

        // Pointer_EnableKeys..Pointer_DfltBtnPrev

        // ch..C_H

        // 3270 terminal keys
        // keysyms::3270_Duplicate => Named::Duplicate,
        // keysyms::3270_FieldMark => Named::FieldMark,
        // keysyms::3270_Right2 => Named::Right2,
        // keysyms::3270_Left2 => Named::Left2,
        // keysyms::3270_BackTab => Named::BackTab,
        keysyms::_3270_EraseEOF => Named::EraseEof,
        // keysyms::3270_EraseInput => Named::EraseInput,
        // keysyms::3270_Reset => Named::Reset,
        // keysyms::3270_Quit => Named::Quit,
        // keysyms::3270_PA1 => Named::Pa1,
        // keysyms::3270_PA2 => Named::Pa2,
        // keysyms::3270_PA3 => Named::Pa3,
        // keysyms::3270_Test => Named::Test,
        keysyms::_3270_Attn => Named::Attn,
        // keysyms::3270_CursorBlink => Named::CursorBlink,
        // keysyms::3270_AltCursor => Named::AltCursor,
        // keysyms::3270_KeyClick => Named::KeyClick,
        // keysyms::3270_Jump => Named::Jump,
        // keysyms::3270_Ident => Named::Ident,
        // keysyms::3270_Rule => Named::Rule,
        // keysyms::3270_Copy => Named::Copy,
        keysyms::_3270_Play => Named::Play,
        // keysyms::3270_Setup => Named::Setup,
        // keysyms::3270_Record => Named::Record,
        // keysyms::3270_ChangeScreen => Named::ChangeScreen,
        // keysyms::3270_DeleteWord => Named::DeleteWord,
        keysyms::_3270_ExSelect => Named::ExSel,
        keysyms::_3270_CursorSelect => Named::CrSel,
        keysyms::_3270_PrintScreen => Named::PrintScreen,
        keysyms::_3270_Enter => Named::Enter,

        keysyms::space => Named::Space,
        // exclam..Sinh_kunddaliya

        // XFree86
        // keysyms::XF86_ModeLock => Named::ModeLock,

        // XFree86 - Backlight controls
        keysyms::XF86_MonBrightnessUp => Named::BrightnessUp,
        keysyms::XF86_MonBrightnessDown => Named::BrightnessDown,
        // keysyms::XF86_KbdLightOnOff => Named::LightOnOff,
        // keysyms::XF86_KbdBrightnessUp => Named::KeyboardBrightnessUp,
        // keysyms::XF86_KbdBrightnessDown => Named::KeyboardBrightnessDown,

        // XFree86 - "Internet"
        keysyms::XF86_Standby => Named::Standby,
        keysyms::XF86_AudioLowerVolume => Named::AudioVolumeDown,
        keysyms::XF86_AudioRaiseVolume => Named::AudioVolumeUp,
        keysyms::XF86_AudioPlay => Named::MediaPlay,
        keysyms::XF86_AudioStop => Named::MediaStop,
        keysyms::XF86_AudioPrev => Named::MediaTrackPrevious,
        keysyms::XF86_AudioNext => Named::MediaTrackNext,
        keysyms::XF86_HomePage => Named::BrowserHome,
        keysyms::XF86_Mail => Named::LaunchMail,
        // keysyms::XF86_Start => Named::Start,
        keysyms::XF86_Search => Named::BrowserSearch,
        keysyms::XF86_AudioRecord => Named::MediaRecord,

        // XFree86 - PDA
        keysyms::XF86_Calculator => Named::LaunchApplication2,
        // keysyms::XF86_Memo => Named::Memo,
        // keysyms::XF86_ToDoList => Named::ToDoList,
        keysyms::XF86_Calendar => Named::LaunchCalendar,
        keysyms::XF86_PowerDown => Named::Power,
        // keysyms::XF86_ContrastAdjust => Named::AdjustContrast,
        // keysyms::XF86_RockerUp => Named::RockerUp,
        // keysyms::XF86_RockerDown => Named::RockerDown,
        // keysyms::XF86_RockerEnter => Named::RockerEnter,

        // XFree86 - More "Internet"
        keysyms::XF86_Back => Named::BrowserBack,
        keysyms::XF86_Forward => Named::BrowserForward,
        // keysyms::XF86_Stop => Named::Stop,
        keysyms::XF86_Refresh => Named::BrowserRefresh,
        keysyms::XF86_PowerOff => Named::Power,
        keysyms::XF86_WakeUp => Named::WakeUp,
        keysyms::XF86_Eject => Named::Eject,
        keysyms::XF86_ScreenSaver => Named::LaunchScreenSaver,
        keysyms::XF86_WWW => Named::LaunchWebBrowser,
        keysyms::XF86_Sleep => Named::Standby,
        keysyms::XF86_Favorites => Named::BrowserFavorites,
        keysyms::XF86_AudioPause => Named::MediaPause,
        // keysyms::XF86_AudioMedia => Named::AudioMedia,
        keysyms::XF86_MyComputer => Named::LaunchApplication1,
        // keysyms::XF86_VendorHome => Named::VendorHome,
        // keysyms::XF86_LightBulb => Named::LightBulb,
        // keysyms::XF86_Shop => Named::BrowserShop,
        // keysyms::XF86_History => Named::BrowserHistory,
        // keysyms::XF86_OpenURL => Named::OpenUrl,
        // keysyms::XF86_AddFavorite => Named::AddFavorite,
        // keysyms::XF86_HotLinks => Named::HotLinks,
        // keysyms::XF86_BrightnessAdjust => Named::BrightnessAdjust,
        // keysyms::XF86_Finance => Named::BrowserFinance,
        // keysyms::XF86_Community => Named::BrowserCommunity,
        keysyms::XF86_AudioRewind => Named::MediaRewind,
        // keysyms::XF86_BackForward => Key::???,
        // XF86_Launch0..XF86_LaunchF

        // XF86_ApplicationLeft..XF86_CD
        keysyms::XF86_Calculater => Named::LaunchApplication2, // Nice typo, libxkbcommon :)
        // XF86_Clear
        keysyms::XF86_Close => Named::Close,
        keysyms::XF86_Copy => Named::Copy,
        keysyms::XF86_Cut => Named::Cut,
        // XF86_Display..XF86_Documents
        keysyms::XF86_Excel => Named::LaunchSpreadsheet,
        // XF86_Explorer..XF86iTouch
        keysyms::XF86_LogOff => Named::LogOff,
        // XF86_Market..XF86_MenuPB
        keysyms::XF86_MySites => Named::BrowserFavorites,
        keysyms::XF86_New => Named::New,
        // XF86_News..XF86_OfficeHome
        keysyms::XF86_Open => Named::Open,
        // XF86_Option
        keysyms::XF86_Paste => Named::Paste,
        keysyms::XF86_Phone => Named::LaunchPhone,
        // XF86_Q
        keysyms::XF86_Reply => Named::MailReply,
        keysyms::XF86_Reload => Named::BrowserRefresh,
        // XF86_RotateWindows..XF86_RotationKB
        keysyms::XF86_Save => Named::Save,
        // XF86_ScrollUp..XF86_ScrollClick
        keysyms::XF86_Send => Named::MailSend,
        keysyms::XF86_Spell => Named::SpellCheck,
        keysyms::XF86_SplitScreen => Named::SplitScreenToggle,
        // XF86_Support..XF86_User2KB
        keysyms::XF86_Video => Named::LaunchMediaPlayer,
        // XF86_WheelButton
        keysyms::XF86_Word => Named::LaunchWordProcessor,
        // XF86_Xfer
        keysyms::XF86_ZoomIn => Named::ZoomIn,
        keysyms::XF86_ZoomOut => Named::ZoomOut,

        // XF86_Away..XF86_Messenger
        keysyms::XF86_WebCam => Named::LaunchWebCam,
        keysyms::XF86_MailForward => Named::MailForward,
        // XF86_Pictures
        keysyms::XF86_Music => Named::LaunchMusicPlayer,

        // XF86_Battery..XF86_UWB
        //
        keysyms::XF86_AudioForward => Named::MediaFastForward,
        // XF86_AudioRepeat
        keysyms::XF86_AudioRandomPlay => Named::RandomToggle,
        keysyms::XF86_Subtitle => Named::Subtitle,
        keysyms::XF86_AudioCycleTrack => Named::MediaAudioTrack,
        // XF86_CycleAngle..XF86_Blue
        //
        keysyms::XF86_Suspend => Named::Standby,
        keysyms::XF86_Hibernate => Named::Hibernate,
        // XF86_TouchpadToggle..XF86_TouchpadOff
        //
        keysyms::XF86_AudioMute => Named::AudioVolumeMute,

        // XF86_Switch_VT_1..XF86_Switch_VT_12

        // XF86_Ungrab..XF86_ClearGrab
        keysyms::XF86_Next_VMode => Named::VideoModeNext,
        // keysyms::XF86_Prev_VMode => Named::VideoModePrevious,
        // XF86_LogWindowTree..XF86_LogGrabInfo

        // SunFA_Grave..SunFA_Cedilla

        // keysyms::SunF36 => Named::F36 | Named::F11,
        // keysyms::SunF37 => Named::F37 | Named::F12,

        // keysyms::SunSys_Req => Named::PrintScreen,
        // The next couple of xkb (until SunStop) are already handled.
        // SunPrint_Screen..SunPageDown

        // SunUndo..SunFront
        keysyms::SUN_Copy => Named::Copy,
        keysyms::SUN_Open => Named::Open,
        keysyms::SUN_Paste => Named::Paste,
        keysyms::SUN_Cut => Named::Cut,

        // SunPowerSwitch
        keysyms::SUN_AudioLowerVolume => Named::AudioVolumeDown,
        keysyms::SUN_AudioMute => Named::AudioVolumeMute,
        keysyms::SUN_AudioRaiseVolume => Named::AudioVolumeUp,
        // SUN_VideoDegauss
        keysyms::SUN_VideoLowerBrightness => Named::BrightnessDown,
        keysyms::SUN_VideoRaiseBrightness => Named::BrightnessUp,
        // SunPowerSwitchShift
        //
        _ => return Key::Unidentified,
    })
}

use iced_runtime::keyboard::{key::Named, Key, Location};

pub fn keysym_location(keysym: u32) -> Location {
    use xkbcommon_dl::keysyms;
    match keysym {
        xkeysym::key::Shift_L
        | keysyms::Control_L
        | keysyms::Meta_L
        | keysyms::Alt_L
        | keysyms::Super_L
        | keysyms::Hyper_L => Location::Left,
        keysyms::Shift_R
        | keysyms::Control_R
        | keysyms::Meta_R
        | keysyms::Alt_R
        | keysyms::Super_R
        | keysyms::Hyper_R => Location::Right,
        keysyms::KP_0
        | keysyms::KP_1
        | keysyms::KP_2
        | keysyms::KP_3
        | keysyms::KP_4
        | keysyms::KP_5
        | keysyms::KP_6
        | keysyms::KP_7
        | keysyms::KP_8
        | keysyms::KP_9
        | keysyms::KP_Space
        | keysyms::KP_Tab
        | keysyms::KP_Enter
        | keysyms::KP_F1
        | keysyms::KP_F2
        | keysyms::KP_F3
        | keysyms::KP_F4
        | keysyms::KP_Home
        | keysyms::KP_Left
        | keysyms::KP_Up
        | keysyms::KP_Right
        | keysyms::KP_Down
        | keysyms::KP_Page_Up
        | keysyms::KP_Page_Down
        | keysyms::KP_End
        | keysyms::KP_Begin
        | keysyms::KP_Insert
        | keysyms::KP_Delete
        | keysyms::KP_Equal
        | keysyms::KP_Multiply
        | keysyms::KP_Add
        | keysyms::KP_Separator
        | keysyms::KP_Subtract
        | keysyms::KP_Decimal
        | keysyms::KP_Divide => Location::Numpad,
        _ => Location::Standard,
    }
}
