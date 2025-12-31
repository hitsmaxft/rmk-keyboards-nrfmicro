//! Shared key position configuration for the Corne keyboard

use rmk::config::PositionalConfig;

/// Creates a positional config based on real hand positions from the matrix_map
///
/// The matrix_map from keyboard_corne.toml shows:
/// ```
/// (0,0, L) (0,1, L) (0,2, L) (0,3, L) (0,4, L) (0,5, L)                        (4,5, R) (4,4, R) (4,3, R) (4,2, R) (4,1, R) (4,0, R)
/// (1,0, L) (1,1, L) (1,2, L) (1,3, L) (1,4, L) (1,5, L)                        (5,5, R) (5,4, R) (5,3, R) (5,2, R) (5,1, R) (5,0, R)
/// (2,0, L) (2,1, L) (2,2, L) (2,3, L) (2,4, L) (2,5, L)                        (6,5, R) (6,4, R) (6,3, R) (6,2, R) (6,1, R) (6,0, R)
///                                        (3,3, L) (3,4, L) (3,5, L)                        (7,5, R) (7,4, R) (7,3, R)
/// ```
///
/// This is mapped to our 8x6 matrix layout where:
/// - Rows 0-2: Left hand keys
/// - Row 3: Only columns 3,4,5 have left hand keys (thumb cluster)
/// - Rows 4-6: Right hand keys
/// - Row 7: Only columns 3,4,5 have right hand keys (thumb cluster)
pub fn create_corne_positional_config() -> PositionalConfig<8, 6> {
    PositionalConfig {
        hand: [
            // Row 0: Left hand keys (0,0-0,5)
            [
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
            ],
            // Row 1: Left hand keys (1,0-1,5)
            [
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
            ],
            // Row 2: Left hand keys (2,0-2,5)
            [
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
            ],
            // Row 3: Only columns 3,4,5 have left hand keys (3,3-3,5), others are Unknown
            [
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
                rmk::config::Hand::Left,
            ],
            // Row 4: Right hand keys (4,5-4,0) mapped to 4,0-4,5 in our matrix
            [
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
            ],
            // Row 5: Right hand keys (5,5-5,0) mapped to 5,0-5,5 in our matrix
            [
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
            ],
            // Row 6: Right hand keys (6,5-6,0) mapped to 6,0-6,5 in our matrix
            [
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
            ],
            // Row 7: Only columns 3,4,5 have right hand keys (7,5-7,3) mapped to 7,3-7,5 in our matrix
            [
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Unknown,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
                rmk::config::Hand::Right,
            ],
        ],
    }
}
