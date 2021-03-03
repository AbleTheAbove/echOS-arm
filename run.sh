#!/bin/bash
cargo xbuild --release && \
qemu-system-aarch64 -machine raspi3 \
  -cpu cortex-a53 \
  -kernel target/aarch64-unknown-none/release/aarch64-bare-metal
