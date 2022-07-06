# chiphuit

Yet another chip8 emulator, written in Rust compiled to WASM.

# play

The emulator is hosted online [here](https://chiphuit.glitch.me/) if you want to give it a try without building from sources, you will have to give the emulator the game you want to play, here's some [chip8 games](https://github.com/kripod/chip8-roms/tree/master/games) to download. Works on touchscreen devices as well.

The emulator also has a debugger view that allows to

* Pause / play the emulator
* See the emulator internal variables
* Save the current VM state in your clipboard in JSON format
* Load a JSON VM state to the emulator in order to replay from a specific state
* Trace the VM state at each CPU cycle
* Save all the traced VM states to your disk in JSON format (WIP)

Modifying internal variables isn't supported yet, but it will be (hopefully) possible in future versions.

| ![The emulator with the debugger](assets/emulator_debugger.png) |
|:--:|
| *The emulator playing Tetris with the debugger view next to it* |


# build & run from sources

## lazy docker way:

```
docker build -t chiphuit -f Dockerfile .
```

```
docker run -p 4000:4000 chiphuit:latest
```

And visit http://127.0.0.1:4000 with your favorite web browser.

## not the lazy docker way:

Make sure you have rust toolchain installed and up to date.

If not, install Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Download the _[wasm-bindgen-cli](https://crates.io/crates/wasm-bindgen-cli)_ and _[basic-http-server](https://crates.io/crates/basic-http-server)_ crates.

```bash
cargo install basic-http-server wasm-bindgen-cli
```

_wasm-bindgen_ and _cargo_ versions must match, make sure _cargo_ is up to date:

```bash
cargo update
```

Add wasm to rustup targets:

```bash
rustup target add wasm32-unknown-unknown
```

Then run,

```
cargo build --release
```

Then run `wasm-bindgen` to generate JS bindings for the wasm file:

```
wasm-bindgen ./target/wasm32-unknown-unknown/release/chiphuit.wasm \
--out-dir build \
--no-typescript \
--target no-modules \
--remove-name-section \
--remove-producers-section \
--omit-default-module-path \
--omit-imports
```

Finally, serve the emulator and play it on your favorite browser @ http://127.0.0.1:4000

```
basic-http-server build/
```

# Demo

Here's a video of the emulator running on an iPhone.

[![The emulator running](https://img.youtube.com/vi/Ix_EGr-9nWQ/maxresdefault.jpg)](https://www.youtube.com/watch?v=Ix_EGr-9nWQ)

# Documentation

Generate & read the documentation of the project

```bash
cargo doc --document-private-items --open
```

Useful links that helped me understand the basics of writing an emulator:

[How to write a chip8 emulator by Laurence Muller](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

[Wikipedia page describing chip8 architecture, opcodes, display etc](https://en.wikipedia.org/wiki/CHIP-8)

[Awesome chip8 rom to test opcodes correctness](https://github.com/corax89/chip8-test-rom)

# Todolist

## debugger
- Write a serde serializer for Emulator from HtmlCollection?
- Write a Message Queue between the Debugger struct and the Emulator struct to allow debugging
- Finish step button -> Make it call emulatore.cycle() on click
- To make it `Emulator` agnostic, it should take as argument a Serialized `Emulator` struct, then `fill_rows` should iterate over the values of the Serialized `Emulator`.

### tracing
- Define Closure & callback (with https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream ?)
- Allow loading trace state into Emulator (Deserialize Trait from html table)

## front
- move all css statements to rust files ? use https://github.com/chinedufn/percy ?
- fix debugger layout in portrait mode

## build
- Allow other compilation targets than WASM with conditional compilation and find a crate to render the screen (egui | wgpu | winit | glfw), or just run the emulator in the terminal?
- package & publish on [wapm](https://wapm.io/)?

## ideas
- Make the clipboard feature work on Webkit
- Fix CSS render in landscape mode for Chrome
- add beep sound with [web_sys::AudioContext](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.AudioContext.html)
- add error handling to code instead of all the wild unwraps
- set FPS / emulator speed during runtime
- add gamepad support with [browser API](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.GamepadEvent.html) not sure this one makes sense for chip8, but it will definitely be useful for future gaming architectures
