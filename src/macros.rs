#[macro_export]
macro_rules! config_matrix_pins_nrf {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new($p.$out_pin, embassy_nrf::gpio::Level::Low, embassy_nrf::gpio::OutputDrive::Standard)), +];
            let input_pins = [$(Input::new($p.$in_pin, embassy_nrf::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}

#[macro_export]
macro_rules! wm {
    ($x: ident, $m: expr) => {
        rmk::types::action::KeyAction::Single(rmk::types::action::Action::KeyWithModifier(
            rmk::types::keycode::KeyCode::$x,
            paste::paste! {rmk::types::modifier::ModifierCombination::[<$m:upper>]},
        ))
    };
}
#[macro_export]
macro_rules! mt {
    ($k: ident, $m: expr) => {
        rmk::types::action::KeyAction::TapHold(
            rmk::types::action::Action::Key(rmk::types::keycode::KeyCode::$k),
            paste::paste! {rmk::types::action::Action::Modifier(rmk::types::modifier::ModifierCombination::[<$m:upper>])},
            rmk::types::action::MorseProfile::const_default(),
        )
    };
}

#[macro_export]
macro_rules! mtp {
    ($k: ident, $m: ident, $p: expr) => {
        paste::paste! {
            rmk::types::action::KeyAction::TapHold(
                rmk::types::action::Action::Key(rmk::types::keycode::KeyCode::$k),
                rmk::types::action::Action::Modifier(rmk::types::modifier::ModifierCombination::[<$m:upper>]),
                $p,
            )
        }
    };
}