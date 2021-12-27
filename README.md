# chiphuit

late night coding trying to code my first emulator (chip8) with rust and webassembly

The goal is to do things with rust/wasm as much as possible rather than JavaScript, using only standard libraries.

![The emulator running](example.png)

useful links that helped me / motivated me a lot:

[Awesome blogpost that motivated me to learn about emulation dev](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

[Wikipedia page describing chip8 architecture, opcodes, display etc](https://en.wikipedia.org/wiki/CHIP-8)

[Awesome chip8 rom to test opcodes](https://github.com/corax89/chip8-test-rom)

## Todolist

### soon

- add gamepad support with [browser API](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.GamepadEvent.html)
- Allow modifying emulator variables
- allow other compilation targets than WASM
- use a Bus structure to handle I/Os instead of bloated RefCells & closures everywhere
- refactooooooor

### osef / pinaillage

- Avoid ROM in string format in loading and directly use [read_as_array_buffer](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.FileReader.html#method.read_as_array_buffer) instead of [read_as_binary_string](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.FileReader.html#method.read_as_binary_string)
- factorize emulator state rendering
- add beep sound with web_sys::{AudioContext, OscillatorType};
- allow tracing opcodes
- responsive CSS, especially for the ROM choice button mdr
- set FPS / emulator speed during runtime
