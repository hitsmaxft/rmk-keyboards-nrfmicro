# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an RMK-based keyboard firmware project targeting nRF52840 microcontrollers (like nice!nano). The project supports split keyboards with BLE connectivity and uses TOML-based configuration files. It includes a custom matrix mapping system that transforms logical keymaps (as they appear in TOML) to electronic pin-based keymaps.

## Build System & Dependencies

### Core Tools
- **cargo-make**: Primary build tool for generating UF2 firmware
- **flip-link**: Stack overflow protection for embedded targets
- **cargo-binutils**: For generating hex files (objcopy)
- **cargo-hex-to-uf2**: Convert hex to UF2 format for bootloader flashing

### Key Dependencies
- **rmk 0.8**: Core keyboard firmware framework from crates.io
- **embassy**: Async runtime for embedded systems
- **nrf-sdc**: Nordic SoftDevice Controller for BLE
- **defmt**: Efficient logging for embedded systems

## Common Build Commands

### Standard Development
```bash
# Check compilation (no flashing needed for code verification)
cargo check --bin central
cargo check --bin peripheral

# Build firmware
cargo build --release

# Generate UF2 files for flashing
cargo make uf2 --release
```

### Environment-Specific Builds
The `Makefile.toml` supports different keyboard profiles:
```bash
# Build for specific keyboard (default: corne)
cargo make --profile corne uf2 --release
cargo make --profile velvet uf2 --release
cargo make --profile keyball uf2 --release
cargo make --profile corne-reset uf2 --release
```

### Flashing to Hardware
```bash
```

## Testing Strategy

### Unit Testing (Host Target Only)

TODO

### Code Verification
```bash
# Check embedded compilation without running
cargo check --bin central --bin peripheral
```

## Architecture Overview

### Split Architecture
The project implements a central/peripheral split keyboard setup:

- **Central (`src/central.rs`)**: Main controller, handles USB/BLE, coordinates with peripheral
- **Peripheral (`src/peripheral.rs`)**: Secondary side, communicates via BLE with central
- **Config-driven variants**: `central_config.rs` and `peripheral_config.rs` use RMK macros for TOML-based configuration

### Key Components

#### Matrix Map System (`src/matrix_map.rs`)
Transforms physical keymaps (sequential order as in TOML) to pin-based keymaps (electronic matrix positions). This allows:
- Defining keymaps in natural layout order
- Automatic transformation to hardware matrix positions
- Support for split keyboards with coordinate parsing

#### Keymap Modules
- `src/keymap.rs`: Main keymap definitions
- `src/keymap42.rs`: 42-key layout support
- `src/keymap_corne.rs`: Corne-specific layouts
- `src/macros.rs`: Custom key action macros (`wm!`, `mt!`, `mtp!`)

#### Configuration
- **TOML files**: `keyboard_*.toml` define hardware configuration
- **Vial support**: JSON files for Vial configuration tool compatibility
- **Environment variables**: Set via `Makefile.toml` profiles

### Memory Configuration
The project uses different memory layouts based on SoftDevice version:
- **S140 6.x**: `FLASH: ORIGIN = 0x00026000, LENGTH = 824K`
- **S140 7.x**: `FLASH: ORIGIN = 0x00027000, LENGTH = 820K`
- Configured in `memory.x`

## Development Workflow

### Code Changes
1. Modify source files or TOML configuration
2. Use `cargo check` for quick compilation verification
3. For matrix/keymap changes, run matrix tests: `./run_matrix_test.sh`
4. Build and test on hardware: `cargo make uf2 --release`

### Hardware Testing
1. Put device in bootloader mode (double-tap reset)
2. Flash firmware: drag UF2 file to mounted drive or use `cargo make flash`
3. Disconnect USB to enable BLE mode (or use switch output key)

### Debugging
- **defmt logging**: Use `info!()`, `debug!()` macros for embedded logging
- **Expansion**: `cargo make expand-main` to see macro expansions
- **RTT logging**: Enabled via defmt-rtt for real-time debugging

## Important Notes

### Cross-Compilation Context
This project cross-compiles for ARM Cortex-M4 (nRF52840). Standard library features and tests are limited to host target development.

### UF2 Bootloader Support
The project is optimized for Adafruit nRF52 bootloader with UF2 support, eliminating the need for debug probes.

### Split Keyboard Coordination
Central and peripheral sides must be flashed with matching firmware. The central handles all USB/BLE host communication.

### Configuration Management
Hardware configuration is primarily done via TOML files rather than direct code modification. The `parse_matrix_map` function bridges TOML configuration with firmware requirements.
