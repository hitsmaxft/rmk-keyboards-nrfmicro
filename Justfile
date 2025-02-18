default:
    @just -l

build: uf2
    @echo "build all targets"

uf2: release
    cargo make uf2

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
