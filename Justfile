default:
    @just -l

build: uf2
    @echo "build all targets"

uf2: release
    cargo make uf2 --release

release:
    cargo build --release

debug:
    cargo build

clean:
    cargo clean
    @rm -f *.uf2 
    @rm -f *.hex

flash-left: build
    pico-dfu -y ./soflev2-rmk-central.uf2 /Volumes/NICENANO
flash-right: build
    pico-dfu -y ./soflev2-rmk-peripheral.uf2 /Volumes/NICENANO

corne:
    export KEYBOARD_TOML_PATH=$(pwd)/keyboard_corne.toml
    export VIAL_JSON_PATH=$(pwd)/vial_corne.json
    just uf2

