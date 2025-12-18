use crate::keymap::{HRM, NUM_LAYER, THUMB_TAP};
use crate::{mt, mtp, wm};
use core::iter::Iterator;
use rmk::types::action::KeyAction;
use rmk::{a, k, ltp, mo, tg, to};

const KEYS: usize = 42;

// Matrix map from your TOML configuration
#[rustfmt::skip]
pub(crate) const MATRIX_MAP: &'static str = "
(0,0, L) (0,1, L) (0,2, L) (0,3, L) (0,4, L) (0,5, L)                        (4,5, R) (4,4, R) (4,3, R) (4,2, R) (4,1, R) (4,0, R)
(1,0, L) (1,1, L) (1,2, L) (1,3, L) (1,4, L) (1,5, L)                        (5,5, R) (5,4, R) (5,3, R) (5,2, R) (5,1, R) (5,0, R)
(2,0, L) (2,1, L) (2,2, L) (2,3, L) (2,4, L) (2,5, L)                        (6,5, R) (6,4, R) (6,3, R) (6,2, R) (6,1, R) (6,0, R)
                           (3,3, L) (3,4, L) (3,5, L)                        (7,5, R) (7,4, R) (7,3, R)
";

/// Create physical keymap arranged in sequential order as defined by matrix map
/// Layout pattern: 12-12-12-6 keys per row (42 total per layer)
/// Physical arrangement follows matrix map sequence:
/// Row 1: L6 + R6(reversed) = 12 keys
/// Row 2: L6 + R6(reversed) = 12 keys
/// Row 3: L6 + R6(reversed) = 12 keys
/// Row 4: L3 + R3(reversed) = 6 keys
#[rustfmt::skip]
pub fn get_physical_keymap() -> [[KeyAction; KEYS]; NUM_LAYER] {
    [
        // Base layer - 42 keys total (12+12+12+6)
        [
            // Row 1: Left 6 keys (Tab, Q, W, E, R, T) + Right 6 keys (P, O, I, U, Y, Backspace) = 12 keys
            k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(P), k!(O), k!(I), k!(U), k!(Y), k!(Backspace),
            // Row 2: Left 6 keys + Right 6 keys = 12 keys
            k!(Escape), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mtp!(D, LGui, HRM), mtp!(F, LShift, HRM), k!(G),
            mtp!(Semicolon, RCtrl, HRM), mtp!(L, RAlt, HRM), mtp!(K, RGui, HRM), mtp!(J, RShift, HRM), k!(H), k!(Enter),
            // Row 3: Left 6 keys + Right 6 keys = 12 keys
            mo!(4), mt!(Z, LGui), k!(X), k!(C), k!(V), k!(B),
            a!(No), k!(N), k!(Dot), k!(Comma), k!(M), k!(Slash),
            // Row 4: Left 3 keys + Right 3 keys = 6 keys (thumb cluster)
            ltp!(6, Escape, THUMB_TAP), mo!(2), k!(Space), ltp!(2, Backspace, THUMB_TAP), mo!(3), ltp!(5, Enter, THUMB_TAP)
        ],
        // Windows layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 2: 12 keys
            a!(No), mtp!(A, LCtrl, HRM), mtp!(S, LAlt, HRM), mt!(D, LCtrl), mtp!(F, LShift, HRM), a!(No),
            mtp!(Semicolon, RCtrl, HRM), mtp!(L, RAlt, HRM), mt!(K, RCtrl), mtp!(J, RShift, HRM), a!(No), a!(No),
            // Row 3: 12 keys
            k!(LGui), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 4: 6 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)
        ],
        // Symbol layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), wm!(Kc0, LShift), wm!(Kc9, LShift), wm!(Kc8, LShift), wm!(Kc7, LShift), wm!(Kc6, LShift), a!(No),
            // Row 2: 12 keys
            a!(No), a!(No), k!(LeftBracket), k!(RightBracket), wm!(LeftBracket, LShift), wm!(RightBracket, LShift),
            wm!(Quote, LShift), wm!(Equal, LShift), k!(Minus), wm!(Minus, LShift), k!(Equal), a!(No),
            // Row 3: 12 keys
            a!(No), a!(No), k!(RightBracket), wm!(Kc9, LShift), wm!(Backslash, LShift), k!(Backslash),
            k!(Quote), wm!(Grave, LShift), k!(Grave), wm!(Slash, LShift), k!(Slash), a!(No),
            // Row 4: 6 keys
            a!(No), a!(No), a!(No), k!(Grave), mo!(4), wm!(Grave, LShift)
        ],
        // Mix layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            k!(LShift), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F10), k!(F9), k!(F8), k!(F7), k!(F6), a!(No),
            // Row 2: 12 keys
            k!(LCtrl), k!(Kc5), k!(Kc4), k!(Kc3), k!(Kc2), k!(Kc1),
            k!(Quote), wm!(Equal, LShift), k!(Equal), k!(Minus), k!(Backspace), a!(No),
            // Row 3: 12 keys
            k!(LGui), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0),
            a!(No), k!(End), k!(Home), k!(Insert), a!(No), a!(No),
            // Row 4: 6 keys
            k!(Tab), mo!(4), k!(Backspace), a!(No), k!(Kc3), a!(No)
        ],
        // Adjust layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            k!(User0), k!(User3), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 2: 12 keys
            k!(User1), k!(User4), a!(No), a!(No), a!(No), a!(No),
            tg!(1), a!(No), a!(No), to!(0), tg!(1), a!(No),
            // Row 3: 12 keys
            k!(User2), k!(User5), k!(User6), k!(User5), a!(No), a!(No),
            a!(No), k!(CapsLock), a!(No), a!(No), a!(No), a!(No),
            // Row 4: 6 keys
            a!(No), a!(No), a!(No), a!(No), k!(Kc4), a!(No)
        ],
        // Navigation layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            a!(No), a!(No), a!(No), k!(MouseUp), k!(PageUp), k!(PageDown),
            k!(MouseWheelDown), k!(MouseWheelUp), k!(MouseBtn2), k!(MouseBtn3), k!(MouseBtn1), a!(No),
            // Row 2: 12 keys
            a!(No), a!(No), k!(MouseLeft), k!(MouseDown), k!(MouseRight), k!(MouseWheelUp),
            a!(No), k!(Right), k!(Up), k!(Down), k!(Left), a!(No),
            // Row 3: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), k!(MouseWheelDown),
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 4: 6 keys
            mo!(7), a!(No), a!(No), a!(No), k!(Kc5), a!(No)
        ],
        // Number/Keypad layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            mo!(7), a!(No), a!(No), a!(No), a!(No), a!(No),
            k!(KpAsterisk), k!(KpMinus), k!(Kp9), k!(Kp8), k!(Kp7), a!(No),
            // Row 2: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            k!(KpSlash), k!(KpPlus), k!(Kp6), k!(Kp5), k!(Kp4), a!(No),
            // Row 3: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            a!(No), a!(No), k!(Kp3), k!(Kp2), k!(Kp1), a!(No),
            // Row 4: 6 keys
            a!(No), a!(No), a!(No), k!(Kp0), k!(KpDot), k!(Kp0)
        ],
        // Debug layer - 42 keys total (12+12+12+6)
        [
            // Row 1: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 2: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
            // Row 3: 12 keys
            a!(No), a!(No), a!(No), a!(No), a!(No), k!(Bootloader), a!(No), a!(No), a!(No), a!(No), k!(Bootloader), a!(No),
            // Row 4: 6 keys
            a!(No), a!(No), a!(No), k!(Kc7), a!(No), a!(No)
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    use core::option::Option::Some;
    use core::result::Result::Ok;
    use core::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct MatrixPosition {
        pub row: u8,
        pub col: u8,
        pub side: char, // 'L' for left, 'R' for right
    }

    /// Parse the matrix map string into a vector of matrix positions
    pub fn parse_matrix_map(matrix_map: &str) -> Vec<MatrixPosition> {
        let mut positions = Vec::new();

        // Use regex-like approach to find all (row,col,side) patterns
        // Since the format has spaces after commas like "(0,0, L)", we need a different approach

        let chars: Vec<char> = matrix_map.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '(' {
                // Look for the matching closing parenthesis
                let mut j = i + 1;
                while j < chars.len() && chars[j] != ')' {
                    j += 1;
                }

                if j < chars.len() && chars[j] == ')' {
                    // Extract the content inside parentheses
                    let content: String = chars[i + 1..j].iter().collect();
                    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

                    if parts.len() == 3 {
                        if let (Ok(row), Ok(col)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>())
                        {
                            let side = parts[2].chars().next().unwrap_or('L');
                            positions.push(MatrixPosition { row, col, side });
                        }
                    }
                    i = j + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        positions
    }

    /// Print the parsed matrix map in a formatted way
    pub fn print_matrix_map(positions: &[MatrixPosition]) {
        println!("Parsed Matrix Map:");
        println!("==================");
        println!("Total positions: {}", positions.len());
        println!();

        // Group by side
        let left_positions: Vec<_> = positions.iter().filter(|p| p.side == 'L').collect();
        let right_positions: Vec<_> = positions.iter().filter(|p| p.side == 'R').collect();

        println!("Left side positions ({}): ", left_positions.len());
        for (i, pos) in left_positions.iter().enumerate() {
            print!("({},{},{})", pos.row, pos.col, pos.side);
            if (i + 1) % 6 == 0 {
                println!(); // New line every 6 positions for better readability
            } else {
                print!(" ");
            }
        }
        if !left_positions.is_empty() {
            println!();
        }

        println!();
        println!("Right side positions ({}): ", right_positions.len());
        for (i, pos) in right_positions.iter().enumerate() {
            print!("({},{},{})", pos.row, pos.col, pos.side);
            if (i + 1) % 6 == 0 {
                println!(); // New line every 6 positions for better readability
            } else {
                print!(" ");
            }
        }
        if !right_positions.is_empty() {
            println!();
        }

        println!();
        println!("Matrix layout visualization:");
        println!("===========================");

        // Create a visual representation
        let max_row = positions.iter().map(|p| p.row).max().unwrap_or(0);
        let max_col = positions.iter().map(|p| p.col).max().unwrap_or(0);

        for row in 0..=max_row {
            // Left side
            for col in 0..=max_col {
                if let Some(_pos) = positions
                    .iter()
                    .find(|p| p.row == row && p.col == col && p.side == 'L')
                {
                    print!("({},{},L) ", row, col);
                } else {
                    print!("        ");
                }
            }

            print!("    "); // Separator between sides

            // Right side (reversed column order for visual alignment)
            for col in (0..=max_col).rev() {
                if let Some(_pos) = positions
                    .iter()
                    .find(|p| p.row == row && p.col == col && p.side == 'R')
                {
                    print!("({},{},R) ", row, col);
                } else {
                    print!("        ");
                }
            }
            println!();
        }
    }

    #[test]
    fn test_matrix_map_parsing() {
        let positions = parse_matrix_map(MATRIX_MAP);

        // Test that we have the expected number of positions (42 for Corne)
        assert_eq!(
            positions.len(),
            42,
            "Should have 42 positions for Corne keyboard"
        );

        // Test some specific positions
        let first_pos = &positions[0];
        assert_eq!(first_pos.row, 0);
        assert_eq!(first_pos.col, 0);
        assert_eq!(first_pos.side, 'L');

        // Count left and right sides
        let left_count = positions.iter().filter(|p| p.side == 'L').count();
        let right_count = positions.iter().filter(|p| p.side == 'R').count();

        assert_eq!(left_count, 21, "Should have 21 left side positions");
        assert_eq!(right_count, 21, "Should have 21 right side positions");

        println!("Matrix map parsing test passed!");
        print_matrix_map(&positions);
    }

    #[test]
    fn test_matrix_structure() {
        let positions = parse_matrix_map(MATRIX_MAP);

        // Verify the physical layout structure
        // Row 1: 6 left + 6 right = 12 keys
        let row0_left = positions
            .iter()
            .filter(|p| p.row == 0 && p.side == 'L')
            .count();
        let row0_right = positions
            .iter()
            .filter(|p| p.row == 4 && p.side == 'R')
            .count();
        assert_eq!(row0_left, 6, "Row 1 should have 6 left keys");
        assert_eq!(row0_right, 6, "Row 1 should have 6 right keys");

        // Row 2: 6 left + 6 right = 12 keys
        let row1_left = positions
            .iter()
            .filter(|p| p.row == 1 && p.side == 'L')
            .count();
        let row1_right = positions
            .iter()
            .filter(|p| p.row == 5 && p.side == 'R')
            .count();
        assert_eq!(row1_left, 6, "Row 2 should have 6 left keys");
        assert_eq!(row1_right, 6, "Row 2 should have 6 right keys");

        // Row 3: 6 left + 6 right = 12 keys
        let row2_left = positions
            .iter()
            .filter(|p| p.row == 2 && p.side == 'L')
            .count();
        let row2_right = positions
            .iter()
            .filter(|p| p.row == 6 && p.side == 'R')
            .count();
        assert_eq!(row2_left, 6, "Row 3 should have 6 left keys");
        assert_eq!(row2_right, 6, "Row 3 should have 6 right keys");

        // Row 4: 3 left + 3 right = 6 keys (thumb cluster)
        let row3_left = positions
            .iter()
            .filter(|p| p.row == 3 && p.side == 'L')
            .count();
        let row3_right = positions
            .iter()
            .filter(|p| p.row == 7 && p.side == 'R')
            .count();
        assert_eq!(
            row3_left, 3,
            "Row 4 should have 3 left keys (thumb cluster)"
        );
        assert_eq!(
            row3_right, 3,
            "Row 4 should have 3 right keys (thumb cluster)"
        );

        println!("Matrix structure test passed!");
        println!("Physical layout verified: 12-12-12-6 keys per row = 42 total keys");
    }
}
