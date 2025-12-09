use rmk::types::action::{EncoderAction, KeyAction, MorseProfile, MorseMode};
use rmk::{a, k, mo, mt, wm, tg, to, mtp, ltp};
use rmk::types::modifier::ModifierCombination;

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
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 0;
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Base layer
        [
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [k!(Escape), mtp!(A, ModifierCombination::LCTRL, HRM), mtp!(S, ModifierCombination::LALT, HRM), mtp!(D, ModifierCombination::LGUI, HRM), mtp!(F, ModifierCombination::LSHIFT, HRM), k!(G)],
            [mo!(4), mt!(Z, ModifierCombination::LGUI), k!(X), k!(C), k!(V), k!(B)],
            [k!(No), k!(No), k!(No), ltp!(6, Escape, THUMB_TAP), mo!(2), k!(Space)],
            [k!(Backspace), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [k!(Enter), k!(H), mtp!(J, ModifierCombination::RSHIFT, HRM), mtp!(K, ModifierCombination::RGUI, HRM), mtp!(L, ModifierCombination::RALT, HRM), mtp!(Semicolon, ModifierCombination::RCTRL, HRM)],
            [k!(Slash), k!(M), k!(Comma), k!(Dot), k!(N), a!(No)],
            [k!(No), k!(No), k!(No), ltp!(5, Enter, THUMB_TAP), mo!(3), ltp!(2, Backspace, THUMB_TAP)]
        ],
        // Windows layer
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), mtp!(A, ModifierCombination::LCTRL, HRM), mtp!(S, ModifierCombination::LALT, HRM), mt!(D, ModifierCombination::LCTRL), mtp!(F, ModifierCombination::LSHIFT, HRM), a!(No)],
            [k!(LGui), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), mtp!(J, ModifierCombination::RSHIFT, HRM), mt!(K, ModifierCombination::RCTRL), mtp!(L, ModifierCombination::RALT, HRM), mtp!(Semicolon, ModifierCombination::RCTRL, HRM)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ],
        // Symbol layer
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), k!(LeftBracket), k!(RightBracket), wm!(LeftBracket, ModifierCombination::LSHIFT), wm!(RightBracket, ModifierCombination::LSHIFT)],
            [a!(No), a!(No), k!(RightBracket), wm!(Kc9, ModifierCombination::LSHIFT), wm!(Backslash, ModifierCombination::LSHIFT), k!(Backslash)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), wm!(Kc6, ModifierCombination::LSHIFT), wm!(Kc7, ModifierCombination::LSHIFT), wm!(Kc8, ModifierCombination::LSHIFT), wm!(Kc9, ModifierCombination::LSHIFT), wm!(Kc0, ModifierCombination::LSHIFT)],
            [a!(No), k!(Equal), wm!(Minus, ModifierCombination::LSHIFT), k!(Minus), wm!(Equal, ModifierCombination::LSHIFT), wm!(Quote, ModifierCombination::LSHIFT)],
            [a!(No), k!(Slash), wm!(Slash, ModifierCombination::LSHIFT), k!(Grave), wm!(Grave, ModifierCombination::LSHIFT), k!(Quote)],
            [a!(No), a!(No), a!(No), wm!(Grave, ModifierCombination::LSHIFT), mo!(4), k!(Grave)]
        ],
        // Mix layer
        [
            [k!(LShift), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5)],
            [k!(LCtrl), k!(Kc5), k!(Kc4), k!(Kc3), k!(Kc2), k!(Kc1)],
            [k!(LGui), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0)],
            [k!(Tab), mo!(4), k!(Backspace), a!(No), a!(No), a!(No)],
            [a!(No), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10)],
            [a!(No), k!(Backspace), k!(Minus), k!(Equal), wm!(Equal, ModifierCombination::LSHIFT), k!(Quote)],
            [a!(No), a!(No), k!(Insert), k!(Home), k!(End), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(Kc3), a!(No)]
        ],
        // Adjust layer
        [
            [k!(User0), k!(User3), a!(No), a!(No), a!(No), a!(No)],
            [k!(User1), k!(User4), a!(No), a!(No), a!(No), a!(No)],
            [k!(User2), k!(User5), k!(User6), k!(User5), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), tg!(1), to!(0), a!(No), a!(No), tg!(1)],
            [a!(No), a!(No), a!(No), a!(No), k!(CapsLock), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(Kc4), a!(No)]
        ],
        // Navigation layer
        [
            [a!(No), a!(No), a!(No), k!(MouseUp), k!(PageUp), k!(PageDown)],
            [a!(No), a!(No), k!(MouseLeft), k!(MouseDown), k!(MouseRight), k!(MouseWheelUp)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(MouseWheelDown)],
            [mo!(7), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(MouseBtn1), k!(MouseBtn3), k!(MouseBtn2), k!(MouseWheelUp), k!(MouseWheelDown)],
            [a!(No), k!(Left), k!(Down), k!(Up), k!(Right), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(Kc5), a!(No)]
        ],
        // Number/Keypad layer
        [
            [mo!(7), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Kp7), k!(Kp8), k!(Kp9), k!(KpMinus), k!(KpAsterisk)],
            [a!(No), k!(Kp4), k!(Kp5), k!(Kp6), k!(KpPlus), k!(KpSlash)],
            [a!(No), k!(Kp1), k!(Kp2), k!(Kp3), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(Kp0), k!(KpDot), k!(Kp0)]
        ],
        // Debug layer
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Bootloader)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Bootloader), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), k!(Kc7)]
        ],
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        []; NUM_LAYER
    ]
}