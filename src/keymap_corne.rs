//! Keymap configuration for Corne keyboard
//!
//! This module defines the keymap for a Corne split keyboard with the layout
//! specified in keyboard_corne.toml. The keymap is defined in physical order
//! and then transformed to electrical matrix positions using the matrix_map.

use rmk::types::action::{EncoderAction, KeyAction, MorseProfile, MorseMode};
use rmk::{a, k, mo, tg, to, ltp};

// Macros (wm!, mt!, mtp!) are available at crate root due to #[macro_export]

// Morse profiles matching those defined in keyboard_corne.toml
const HRM: MorseProfile = MorseProfile::new(
    Some(true), // unilateral_tap
    Some(MorseMode::PermissiveHold),
    Some(250u16), // hold_timeout
    Some(250u16), // gap_timeout
);

const THUMB_TAP: MorseProfile = MorseProfile::new(
    Some(false), // unilateral_tap
    Some(MorseMode::HoldOnOtherPress),
    Some(250u16), // hold_timeout
    Some(250u16), // gap_timeout
);

// Matrix configuration from keyboard_corne.toml
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 0;

// Matrix map from keyboard_corne.toml
const MATRIX_MAP: &str = r#"
(0,0, L) (0,1, L) (0,2, L) (0,3, L) (0,4, L) (0,5, L)                        (4,5, R) (4,4, R) (4,3, R) (4,2, R) (4,1, R) (4,0, R)
(1,0, L) (1,1, L) (1,2, L) (1,3, L) (1,4, L) (1,5, L)                        (5,5, R) (5,4, R) (5,3, R) (5,2, R) (5,1, R) (5,0, R)
(2,0, L) (2,1, L) (2,2, L) (2,3, L) (2,4, L) (2,5, L)                        (6,5, R) (6,4, R) (6,3, R) (6,2, R) (6,1, R) (6,0, R)
                           (3,3, L) (3,4, L) (3,5, L)                        (7,5, R) (7,4, R) (7,3, R)
"#;

/// Get the physical keymap in sequential order as defined in TOML
///
/// This function returns the keymap arranged in the sequential order as it appears
/// in the TOML configuration file, reading left-to-right, top-to-bottom.
/// The layout has 42 keys total: 6x3 + 3 thumbs per side.
#[rustfmt::skip]
pub const fn get_physical_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0 - Base
        [
            // Left side row 1: Tab Q W E R T
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            // Right side row 1: Y U I O P Backslash
            [k!(Y), k!(U), k!(I), k!(O), k!(P), k!(Backslash)],
            // Left side row 2: Escape MT(A,LCtrl,HRM) MT(S,LAlt,HRM) MT(D,LGui,HRM) MT(F,LShift,HRM) G
            [k!(Escape), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mtp!(D, LGui, HRM), mtp!(F, LShift, HRM), k!(G)],
            // Right side row 2: H MT(J,RShift,HRM) MT(K,RGui,HRM) MT(L,RAlt,HRM) MT(Semicolon,RCtrl,HRM) Quote
            [k!(H), mtp!(J, RShift, HRM), mtp!(K, RGui, HRM), mtp!(L, RAlt, HRM), mtp!(Semicolon, RCtrl, HRM), k!(Quote)],
            // Left side row 3: MO(4) MT(Z,LGui) X C V B
            [mo!(4), mt!(Z, LGui), k!(X), k!(C), k!(V), k!(B)],
            // Right side row 3: N M Comma Dot Slash Backspace
            [k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), k!(Backspace)],
            // Left thumbs: LT(6,Escape,ThumbTap) MO(2) Space
            [a!(No), a!(No), a!(No), ltp!(6, Escape, THUMB_TAP), mo!(2), k!(Space)],
            // Right thumbs: LT(5,Enter,ThumbTap) MO(3) LT(2,Backspace,ThumbTap)
            [a!(No), a!(No), a!(No), ltp!(5, Enter, THUMB_TAP), mo!(3), ltp!(2, Backspace, THUMB_TAP)]
        ],
        // Layer 1 - Windows
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mt!(D, LCtrl), mtp!(F, LShift, HRM), a!(No)],
            [a!(No), mtp!(J, RShift, HRM), mt!(K, RCtrl), mtp!(L, RAlt, HRM), mtp!(Semicolon, RCtrl, HRM), a!(No)],
            [k!(LGui), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ],
        // Layer 2 - Symbol
        [
            [a!(No), wm!(Kc1, LShift), wm!(Kc2, LShift), wm!(Kc3, LShift), wm!(Kc4, LShift), wm!(Kc5, LShift)],
            [wm!(Kc6, LShift), wm!(Kc7, LShift), wm!(Kc8, LShift), wm!(Kc9, LShift), wm!(Kc0, LShift), wm!(Backslash, LShift)],
            [a!(No), a!(No), k!(LeftBracket), k!(RightBracket), wm!(LeftBracket, LShift), wm!(RightBracket, LShift)],
            [k!(Equal), wm!(Minus, LShift), k!(Minus), wm!(Equal, LShift), wm!(Quote, LShift), wm!(Semicolon, LShift)],
            [a!(No), a!(No), k!(RightBracket), wm!(Kc9, LShift), wm!(Backslash, LShift), k!(Backslash)],
            [k!(Slash), wm!(Slash, LShift), k!(Grave), wm!(Grave, LShift), k!(Quote), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), wm!(Grave, LShift), mo!(4), k!(Grave)]
        ],
        // Layer 3 - Mix/Raise
        [
            [k!(LShift), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5)],
            [k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), k!(F11)],
            [k!(LCtrl), k!(Kc5), k!(Kc4), k!(Kc3), k!(Kc2), k!(Kc1)],
            [k!(Backspace), k!(Minus), k!(Equal), wm!(Equal, LShift), k!(Quote), k!(F12)],
            [k!(LGui), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0)],
            [k!(Tab), k!(Insert), k!(Home), k!(End), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(Tab), mo!(4), k!(Backspace)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Kc3)]
        ],
        // Layer 4 - Adjust
        [
            [k!(User0), k!(User4), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(User1), k!(User3), a!(No), a!(No), a!(No), a!(No)],
            [tg!(1), to!(0), a!(No), a!(No), a!(No), tg!(1)],
            [k!(User2), k!(User5), k!(User6), k!(User5), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(CapsLock), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Kc4)]
        ],
        // Layer 5 - Navigation
        [
            [a!(No), a!(No), a!(No), k!(MouseUp), k!(PageUp), k!(PageDown)],
            [k!(MouseBtn1), k!(MouseBtn3), k!(MouseBtn2), k!(MouseWheelUp), k!(MouseWheelDown), a!(No)],
            [a!(No), a!(No), k!(MouseLeft), k!(MouseDown), k!(MouseRight), k!(MouseWheelUp)],
            [k!(Left), k!(Down), k!(Up), k!(Right), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(MouseWheelDown)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), mo!(7), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Kc5)]
        ],
        // Layer 6 - Number/Keypad
        [
            [mo!(7), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp7), k!(Kp8), k!(Kp9), k!(KpMinus), k!(KpAsterisk), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp4), k!(Kp5), k!(Kp6), k!(KpPlus), k!(KpSlash), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp1), k!(Kp2), k!(Kp3), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(Kp0), k!(KpDot), k!(Kp0)]
        ],
        // Layer 7 - Debug
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Bootloader)],
            [k!(Bootloader), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Kc7)]
        ],
    ]
}

/// Get the default keymap with keys arranged by electrical matrix positions
///
/// This function returns the keymap with keys arranged according to the electrical
/// matrix positions as specified by the matrix_map in keyboard_corne.toml.
/// The transformation maps physical sequence to electrical positions:
/// Physical pos 0-5   → Matrix (0,0)-(0,5): Tab Q W E R T
/// Physical pos 6-11  → Matrix (4,5)-(4,0): Y U I O P Backslash (reversed cols)
/// Physical pos 12-17 → Matrix (1,0)-(1,5): Escape A S D F G
/// Physical pos 18-23 → Matrix (5,5)-(5,0): H J K L ; Quote (reversed cols)
/// Physical pos 24-29 → Matrix (2,0)-(2,5): MO(4) Z X C V B
/// Physical pos 30-35 → Matrix (6,5)-(6,0): N M , . / Backspace (reversed cols)
/// Physical pos 36-38 → Matrix (3,3)-(3,5): LT(6,Esc,TT) MO(2) Space
/// Physical pos 39-41 → Matrix (7,5)-(7,3): LT(5,Enter,TT) MO(3) LT(2,BS,TT) (reversed cols)
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0 - Base
        [
            // Row 0: Physical pos 0-5 → (0,0)-(0,5): Tab Q W E R T
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            // Row 1: Physical pos 12-17 → (1,0)-(1,5): Escape A S D F G
            [k!(Escape), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mtp!(D, LGui, HRM), mtp!(F, LShift, HRM), k!(G)],
            // Row 2: Physical pos 24-29 → (2,0)-(2,5): MO(4) Z X C V B
            [mo!(4), mt!(Z, LGui), k!(X), k!(C), k!(V), k!(B)],
            // Row 3: Physical pos 36-38 → (3,3)-(3,5): LT(6,Escape,ThumbTap) MO(2) Space
            [a!(No), a!(No), a!(No), ltp!(6, Escape, THUMB_TAP), mo!(2), k!(Space)],
            // Row 4: Physical pos 6-11 → (4,5)-(4,0): Y U I O P Backslash (REVERSED)
            [k!(Backslash), k!(P), k!(O), k!(I), k!(U), k!(Y)],
            // Row 5: Physical pos 18-23 → (5,5)-(5,0): H J K L ; Quote (REVERSED)
            [k!(Quote), mtp!(Semicolon, RCtrl, HRM), mtp!(L, RAlt, HRM), mtp!(K, RGui, HRM), mtp!(J, RShift, HRM), k!(H)],
            // Row 6: Physical pos 30-35 → (6,5)-(6,0): N M , . / Backspace (REVERSED)
            [k!(Backspace), k!(Slash), k!(Dot), k!(Comma), k!(M), k!(N)],
            // Row 7: Physical pos 39-41 → (7,5)-(7,3): LT(5,Enter,ThumbTap) MO(3) LT(2,Backspace,ThumbTap) (REVERSED)
            [a!(No), a!(No), a!(No), ltp!(2, Backspace, THUMB_TAP), mo!(3), ltp!(5, Enter, THUMB_TAP)]
        ],
        // Layer 1 - Windows (following same transformation pattern)
        [
            // Row 0: __ __ __ __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 1: __ MT(A,LCtrl) MT(S,LAlt) MT(D,LCtrl) MT(F,LShift) __
            [a!(No), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mt!(D, LCtrl), mtp!(F, LShift, HRM), a!(No)],
            // Row 2: LGui __ __ __ __ __
            [k!(LGui), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 3: __ __ __ (thumbs transparent)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 4: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 5: __ MT(Semicolon,RCtrl) MT(L,RAlt) MT(K,RCtrl) MT(J,RShift) __ (REVERSED)
            [a!(No), mtp!(Semicolon, RCtrl, HRM), mtp!(L, RAlt, HRM), mt!(K, RCtrl), mtp!(J, RShift, HRM), a!(No)],
            // Row 6: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 7: __ __ __ (thumbs transparent, REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ],
        // Layer 2 - Symbol (following transformation pattern)
        [
            // Row 0: __ WM(Kc1,LShift) WM(Kc2,LShift) WM(Kc3,LShift) WM(Kc4,LShift) WM(Kc5,LShift)
            [a!(No), wm!(Kc1, LShift), wm!(Kc2, LShift), wm!(Kc3, LShift), wm!(Kc4, LShift), wm!(Kc5, LShift)],
            // Row 1: __ __ LeftBracket RightBracket WM(LeftBracket,LShift) WM(RightBracket,LShift)
            [a!(No), a!(No), k!(LeftBracket), k!(RightBracket), wm!(LeftBracket, LShift), wm!(RightBracket, LShift)],
            // Row 2: __ __ RightBracket WM(Kc9,LShift) WM(Backslash,LShift) Backslash
            [a!(No), a!(No), k!(RightBracket), wm!(Kc9, LShift), wm!(Backslash, LShift), k!(Backslash)],
            // Row 3: __ __ __ (thumbs transparent)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 4: WM(Kc6,LShift) WM(Kc7,LShift) WM(Kc8,LShift) WM(Kc9,LShift) WM(Kc0,LShift) WM(Backslash,LShift) (REVERSED)
            [wm!(Backslash, LShift), wm!(Kc0, LShift), wm!(Kc9, LShift), wm!(Kc8, LShift), wm!(Kc7, LShift), wm!(Kc6, LShift)],
            // Row 5: Equal WM(Minus,LShift) Minus WM(Equal,LShift) WM(Quote,LShift) WM(Semicolon,LShift) (REVERSED)
            [wm!(Semicolon, LShift), wm!(Quote, LShift), wm!(Equal, LShift), k!(Minus), wm!(Minus, LShift), k!(Equal)],
            // Row 6: Slash WM(Slash,LShift) Grave WM(Grave,LShift) Quote __ (REVERSED)
            [a!(No), k!(Quote), wm!(Grave, LShift), k!(Grave), wm!(Slash, LShift), k!(Slash)],
            // Row 7: WM(Grave,LShift) MO(4) Grave (REVERSED)
            [a!(No), a!(No), a!(No), k!(Grave), mo!(4), wm!(Grave, LShift)]
        ],
        // Layer 3 - Mix/Raise
        [
            // Row 0: LShift F1 F2 F3 F4 F5
            [k!(LShift), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5)],
            // Row 1: LCtrl Kc5 Kc4 Kc3 Kc2 Kc1
            [k!(LCtrl), k!(Kc5), k!(Kc4), k!(Kc3), k!(Kc2), k!(Kc1)],
            // Row 2: LGui Kc6 Kc7 Kc8 Kc9 Kc0
            [k!(LGui), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0)],
            // Row 3: Tab MO(4) Backspace
            [a!(No), a!(No), a!(No), k!(Tab), mo!(4), k!(Backspace)],
            // Row 4: F6 F7 F8 F9 F10 F11 (REVERSED)
            [k!(F11), k!(F10), k!(F9), k!(F8), k!(F7), k!(F6)],
            // Row 5: Backspace Minus Equal WM(Equal,LShift) Quote F12 (REVERSED)
            [k!(F12), k!(Quote), wm!(Equal, LShift), k!(Equal), k!(Minus), k!(Backspace)],
            // Row 6: Tab Insert Home End __ __ (REVERSED)
            [a!(No), a!(No), k!(End), k!(Home), k!(Insert), k!(Tab)],
            // Row 7: __ __ Kc3 (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kc3), a!(No), a!(No)]
        ],
        // Layer 4 - Adjust
        [
            // Row 0: @Bt1 @BtPre __ __ __ __
            [k!(User0), k!(User4), a!(No), a!(No), a!(No), a!(No)],
            // Row 1: @Bt2 @BtNext __ __ __ __
            [k!(User1), k!(User3), a!(No), a!(No), a!(No), a!(No)],
            // Row 2: @Bt3 @BtClear @BtUsb @BtClear __ __
            [k!(User2), k!(User5), k!(User6), k!(User5), a!(No), a!(No)],
            // Row 3: __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 4: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 5: TG(1) TO(0) __ __ __ TG(1) (REVERSED)
            [tg!(1), a!(No), a!(No), a!(No), to!(0), tg!(1)],
            // Row 6: __ __ __ __ __ CapsLock (REVERSED)
            [k!(CapsLock), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 7: __ __ Kc4 (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kc4), a!(No), a!(No)]
        ],
        // Layer 5 - Navigation
        [
            // Row 0: __ __ __ MouseUp PageUp PageDown
            [a!(No), a!(No), a!(No), k!(MouseUp), k!(PageUp), k!(PageDown)],
            // Row 1: __ __ MouseLeft MouseDown MouseRight MouseWheelUp
            [a!(No), a!(No), k!(MouseLeft), k!(MouseDown), k!(MouseRight), k!(MouseWheelUp)],
            // Row 2: __ __ __ __ __ MouseWheelDown
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(MouseWheelDown)],
            // Row 3: MO(7) __ __
            [a!(No), a!(No), a!(No), mo!(7), a!(No), a!(No)],
            // Row 4: MouseBtn1 MouseBtn3 MouseBtn2 MouseWheelUp MouseWheelDown __ (REVERSED)
            [a!(No), k!(MouseWheelDown), k!(MouseWheelUp), k!(MouseBtn2), k!(MouseBtn3), k!(MouseBtn1)],
            // Row 5: Left Down Up Right __ __ (REVERSED)
            [a!(No), a!(No), k!(Right), k!(Up), k!(Down), k!(Left)],
            // Row 6: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 7: __ __ Kc5 (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kc5), a!(No), a!(No)]
        ],
        // Layer 6 - Number/Keypad
        [
            // Row 0: MO(7) __ __ __ __ __
            [mo!(7), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 1: __ __ __ __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 2: __ __ __ __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 3: __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 4: Kp7 Kp8 Kp9 KpMinus KpAsterisk __ (REVERSED)
            [a!(No), k!(KpAsterisk), k!(KpMinus), k!(Kp9), k!(Kp8), k!(Kp7)],
            // Row 5: Kp4 Kp5 Kp6 KpPlus KpSlash __ (REVERSED)
            [a!(No), k!(KpSlash), k!(KpPlus), k!(Kp6), k!(Kp5), k!(Kp4)],
            // Row 6: Kp1 Kp2 Kp3 __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kp3), k!(Kp2), k!(Kp1)],
            // Row 7: Kp0 KpDot Kp0 (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kp0), k!(KpDot), k!(Kp0)]
        ],
        // Layer 7 - Debug
        [
            // Row 0: __ __ __ __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 1: __ __ __ __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 2: __ __ __ __ __ Bootloader
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Bootloader)],
            // Row 3: __ __ __
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 4: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 5: __ __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Row 6: Bootloader __ __ __ __ __ (REVERSED)
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Bootloader)],
            // Row 7: __ __ Kc7 (REVERSED)
            [a!(No), a!(No), a!(No), k!(Kc7), a!(No), a!(No)]
        ],
    ]
}

/// Get the default encoder map (empty for Corne)
pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        []; NUM_LAYER
    ]
}