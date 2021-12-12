late night coding trying to code my first emulator (chip8) with rust and webassembly

![The emulator running](example.png)

useful docs:

https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
https://en.wikipedia.org/wiki/CHIP-8
https://github.com/corax89/chip8-test-rom

TODO:

- Get rid of hardcoded ROM, and launch emulator once the user game is loaded in memory
- Allow modifying emulator variables
- factorize emulator state rendering
- add beep sound with web_sys::{AudioContext, OscillatorType};
- add gamepad support with APIs https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.GamepadEvent.html & https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent
- allow other compilation targets than WASM
- allow tracing opcodes
