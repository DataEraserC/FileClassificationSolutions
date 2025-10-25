# Cross Compilation Help Guide

## For i686-unknown-linux-gnu target

You need to install gcc-multilib:
```bash
sudo apt-get install gcc-multilib
```

## For ARM targets (aarch64-unknown-linux-gnu, armv7-unknown-linux-gnueabihf)

You need to install cross-compilation tools:
```bash
# For aarch64
sudo apt-get install gcc-aarch64-linux-gnu

# For armv7
sudo apt-get install gcc-arm-linux-gnueabihf
```

## For RISCV targets

Note: riscv32i-unknown-none-elf does not support std, so it's not suitable for this project.
The riscv64gc-unknown-linux-gnu target requires RISC-V cross-compilation tools:
```bash
sudo apt-get install gcc-riscv64-linux-gnu
```

## Using Cross-rs

Consider using the `cross` tool for easier cross-compilation:
```bash
cargo install cross
cross build --target <target-triple>
```

Supported targets with cross:
- i686-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- armv7-unknown-linux-gnueabihf
- riscv64gc-unknown-linux-gnu

Note: riscv32i-unknown-none-elf is not appropriate for this application since it doesn't support std.