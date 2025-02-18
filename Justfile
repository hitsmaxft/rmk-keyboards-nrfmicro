default:
    echo "helo"

build:
    cargo make uf2
release:
    cargo build --release
debug:
    cargo build --release

clean:
    cargo clean
    rm -f *.uf2 
    rm -f *.hex

lflash:
    pico-dfu -y ./soflev2-rmk-central.uf2 /Volumes/NICENANO
lright:
    pico-dfu -y ./soflev2-rmk-peripheral.uf2 /Volumes/NICENANO
