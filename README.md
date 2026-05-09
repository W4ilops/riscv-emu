riscv-emu is a 64-bit RISC-V cpu emulator in Rust.
Implements RV64I and Zicsr.
Memory-mapped bus.
UART at 0x10000000.
RAM at 0x80000000.
Time-travel debugger records execution to a .trace file.
Replay can seek to any cycle.
CLI has run, record, replay subcommands.
Built to eventually run xv6.
17 tests passing.

cargo run -- run --binary path/to.bin
cargo run -- record --binary path/to.bin --output out.trace
cargo run -- replay --trace out.trace
cargo test

Contributing is fine, just open a PR.

MIT
