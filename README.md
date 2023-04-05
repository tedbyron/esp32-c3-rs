# esp32-c3-rs

```shell
rustup toolchain install nightly-2023-04-04 --component rust-src
cargo install espflash ldproxy

sudo apt install git llvm-dev libclang-dev clang gcc ninja-build cmake ccache libudev-dev python3 \
     python3-pip python3-venv libusb-1.0-0 libffi-dev libssl-dev pkg-config libtinfo5 flex bison gperf \
     dfu-util libusb-1.0-0 # 🫤

pushd ~/.espressif/esp-idf/v4.4.1/
./install.sh
source export.sh
popd

cargo b --release -p hello-board
espflash --monitor target/riscv32imc-esp-espidf/release/hello-board
```
