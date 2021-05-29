# This runs the kernel on the qemu virt machine
cargo xbuild --release &&
qemu-system-aarch64 -machine virt \
-cpu cortex-a53 \
-display none \
-serial stdio  \
-kernel target/aarch64-unknown-none/release/aarch64-bare-metal
