//! Matrix map parsing and transformation utilities
//!
//! This module provides functionality to parse matrix_map strings from TOML configuration
//! and transform physical keymaps (arranged sequentially) to pin-based keymaps
//! (arranged by electronic matrix positions).

#![allow(unused_imports)]

use rmk::types::action::KeyAction;

// Import necessary core items for no_std compatibility
use core::{option::Option::{Some, None}, result::Result::{Ok, Err}, iter::Iterator, option::Option};

/// Parses a matrix_map string and transforms a physical keymap to pin keymap array
///
/// This function converts a keymap arranged in sequential order (as defined in TOML config)
/// to a keymap arranged by electronic pin positions (as needed by the firmware).
///
/// # Arguments
/// * `matrix_map` - A string containing coordinates in the format "(row,col,hand)"
/// * `keymap_physical` - A slice of flattened KeyAction arrays representing the physical keymap
///
/// # Returns
/// A 3D array of KeyAction representing the pin keymap organized by [layer][row][col]
///
/// # Example
/// Given matrix_map="(0,0,L) (0,1,L) (1,0,L) (1,1,L)" and physical keymap:
/// ```
/// [
///     [k!(A), k!(B), k!(C), k!(D)],  // Keys A, B, C, D in sequence (flattened)
/// ]
/// ```
///
/// The function produces:
/// ```
/// [
///     [k!(A), k!(B)],  // A at (0,0), B at (0,1)
///     [k!(C), k!(D)],  // C at (1,0), D at (1,1)
/// ]
/// ```
pub fn parse_matrix_map<const ROW: usize, const COL: usize, const NUM_LAYER: usize, const KEYS: usize>(
    matrix_map: &str,
    keymap_physical: &[[KeyAction; KEYS]; NUM_LAYER]
) -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    // Initialize the result array with default values
    let mut pin_keymap = [[[KeyAction::No; COL]; ROW]; NUM_LAYER];

    // Parse coordinate tuples from the matrix_map string
    // Use a helper function to calculate the array size at compile time
    let coordinates = parse_coordinates_for_matrix::<ROW, COL>(matrix_map);

    // Transform each layer using the parsed coordinates
    for layer_idx in 0..NUM_LAYER {
        for (seq_idx, (row, col)) in coordinates.iter().enumerate() {
            // Skip invalid coordinates (0,0 pairs that weren't set)
            if *row == 0 && *col == 0 && seq_idx > 0 {
                continue;
            }

            // Use the flattened keymap structure - seq_idx directly maps to the physical position
            if seq_idx < KEYS && (*row as usize) < ROW && (*col as usize) < COL {
                let key_action = keymap_physical[layer_idx][seq_idx];
                pin_keymap[layer_idx][*row as usize][*col as usize] = key_action;
            }
        }
    }

    pin_keymap
}

/// Parses coordinate tuples from matrix_map string
///
/// Extracts (row, col) pairs from a string containing coordinates in the format:
/// "(row,col,hand) (row,col,hand) ..."
///
/// # Arguments
/// * `matrix_map` - String containing coordinate tuples
///
/// # Returns
/// Array of (row, col) coordinate pairs in sequence order (limited to MAX_COORDS)
///
/// # Example
/// ```
/// let matrix_map = "(0,0,L) (0,1,L) (1,0,L) (1,1,L)";
/// let coords: [(u8, u8); 4] = parse_coordinates(matrix_map);
/// // coords will contain [(0, 0), (0, 1), (1, 0), (1, 1)]
/// ```
pub fn parse_coordinates<const MAX_COORDS: usize>(matrix_map: &str) -> [(u8, u8); MAX_COORDS] {
    let mut coordinates = [(0u8, 0u8); MAX_COORDS];
    let mut coord_count = 0;
    let mut i = 0;
    let bytes = matrix_map.as_bytes();

    while i < bytes.len() && coord_count < MAX_COORDS {
        // Look for opening parenthesis
        if bytes[i] == b'(' {
            // Find closing parenthesis
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b')' {
                j += 1;
            }

            if j < bytes.len() {
                // Extract content between parentheses
                let content = &matrix_map[i+1..j];

                // Parse the coordinate tuple
                if let Some((row, col)) = parse_coordinate_tuple(content) {
                    coordinates[coord_count] = (row, col);
                    coord_count += 1;
                }

                i = j + 1;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    coordinates
}

/// Helper function to parse coordinates for a specific matrix size
///
/// This works around the limitation that const generic expressions cannot be used
/// in array size specifications.
fn parse_coordinates_for_matrix<const ROW: usize, const COL: usize>(matrix_map: &str) -> [(u8, u8); MAX_MATRIX_SIZE] {
    // Use a fixed maximum size that should accommodate most matrices
    // This is a compromise to work with const generic limitations
    parse_coordinates::<MAX_MATRIX_SIZE>(matrix_map)
}

/// Maximum matrix size (should be >= largest expected ROW * COL)
const MAX_MATRIX_SIZE: usize = 42;

/// Parses a single coordinate tuple from content like "row,col,hand"
///
/// # Arguments
/// * `content` - String slice containing "row,col,hand" format
///
/// # Returns
/// Option<(u8, u8)> - Some((row, col)) if parsing succeeds, None otherwise
fn parse_coordinate_tuple(content: &str) -> Option<(u8, u8)> {
    let mut parts = content.split(',');

    if let (Some(row_str), Some(col_str)) = (parts.next(), parts.next()) {
        let row_str = row_str.trim();
        let col_str = col_str.trim();

        if let (Ok(row), Ok(col)) = (row_str.parse::<u8>(), col_str.parse::<u8>()) {
            return Some((row, col));
        }
    }

    None
}

// Tests are only available when building for standard library targets
// This avoids issues with no_std embedded targets that don't have the test crate
#[cfg(all(test, not(target_os = "none")))]
mod tests {
    use super::*;
    use rmk::{k};

    #[test]
    fn test_simple_2x2_matrix() {
        let matrix_map = "(0,0,L) (0,1,L) (1,0,L) (1,1,L)";

        // Physical keymap: keys arranged in sequential order (flattened structure)
        let keymap_physical: [[KeyAction; 4]; 1] = [
            [k!(A), k!(B), k!(C), k!(D)]  // Sequence: A=pos 0, B=pos 1, C=pos 2, D=pos 3
        ];

        let pin_keymap = parse_matrix_map::<2, 2, 1, 4>(&matrix_map, &keymap_physical);

        // Verify the transformation:
        // Matrix coordinate (0,0) should get sequence position 0 -> A
        // Matrix coordinate (0,1) should get sequence position 1 -> B
        // Matrix coordinate (1,0) should get sequence position 2 -> C
        // Matrix coordinate (1,1) should get sequence position 3 -> D
        assert_eq!(pin_keymap[0][0][0], k!(A)); // Position (0,0) -> A
        assert_eq!(pin_keymap[0][0][1], k!(B)); // Position (0,1) -> B
        assert_eq!(pin_keymap[0][1][0], k!(C)); // Position (1,0) -> C
        assert_eq!(pin_keymap[0][1][1], k!(D)); // Position (1,1) -> D
    }

    #[test]
    fn test_coordinate_parsing() {
        let matrix_map = "(0,0,L) (1,2,R) (3,4,L) (7,5,R)";
        let coordinates: [(u8, u8); 4] = parse_coordinates(matrix_map);

        // Check the first few coordinates
        assert_eq!(coordinates[0], (0, 0));
        assert_eq!(coordinates[1], (1, 2));
        assert_eq!(coordinates[2], (3, 4));
        assert_eq!(coordinates[3], (7, 5));
    }

    #[test]
    fn test_coordinate_parsing_with_whitespace() {
        let matrix_map = "(0, 0, L)  (1,2,R)\n(3, 4, L)\t(7,5,R)";
        let coordinates: [(u8, u8); 4] = parse_coordinates(matrix_map);

        // Check the first few coordinates
        assert_eq!(coordinates[0], (0, 0));
        assert_eq!(coordinates[1], (1, 2));
        assert_eq!(coordinates[2], (3, 4));
        assert_eq!(coordinates[3], (7, 5));
    }

    #[test]
    fn test_coordinate_tuple_parsing() {
        // Test individual coordinate tuple parsing
        assert_eq!(parse_coordinate_tuple("0,0,L"), Some((0, 0)));
        assert_eq!(parse_coordinate_tuple("1,2,R"), Some((1, 2)));
        assert_eq!(parse_coordinate_tuple(" 3, 4, L "), Some((3, 4)));
        assert_eq!(parse_coordinate_tuple("invalid"), None);
        assert_eq!(parse_coordinate_tuple(",3,L"), None);
    }
}