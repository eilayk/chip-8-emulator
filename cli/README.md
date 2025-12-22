# CLI Player

This folder contains a CLI interface for running the chip-8 emulator from the command line.

## Example Usage

```fish
cargo run test.rom
```

You can also specify the clock speed (instructions per second):

```fish
cargo run test.rom --clock-speed 500
```

### Controls
- Use keys: `1234 QWER ASDF ZXCV` to interact with the emulator (mapped to CHIP-8 keypad)
- Press `Esc` or `Ctrl+C` to quit