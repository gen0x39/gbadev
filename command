rustc --target=arm-none-eabi.json --crate-type=rlib -C opt-level=2 -C no-prepopulate-passes -Z no-landing-pads -o build/libcore.rlib ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/lib.rs



rustc --target=arm-none-eabi.json --crate-type=rlib -C opt-level=2 -C no-prepopulate-passes -o build/libcore.rlib ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/lib.rs