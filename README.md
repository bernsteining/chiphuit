# wkrakmi

A Rust program to generate lightweight and standalone WebAssembly crackmes for Capture The Flag platforms.


## Usage

Modify the *challenge* function in *[challenge.rs](https://github.com/bernsteining/wkrakmi/blob/master/src/challenge.rs)* in order to choose your own challenge difficulty.

Make sure you have rust toolchain installed and up to date.

If not, install Rust toolchain:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Download the *[wasm-bindgen-cli](https://crates.io/crates/wasm-bindgen-cli)* and *[basic-http-server](https://crates.io/crates/basic-http-server)* crates.

```
cargo install basic-http-server wasm-bindgen-cli
```

*wasm-bindgen* and *cargo* versions must match, make sure *cargo* is up to date:


```
cargo update
```

Add wasm to rustup targets:

```
rustup target add wasm32-unknown-unknown
```

Then, run the *build.sh* script. It will compile the Rust code to WebAssembly, and run the challenge on 127.0.0.1:4000.

If you don't want to download *basic-http-server* with *cargo*, you can still run the challenge with the following Python script:

```
import http.server
import socketserver

PORT = 4000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
    '.wasm': 'application/wasm',
})

socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    httpd.allow_reuse_address = True
    print("serving at port", PORT)
    httpd.serve_forever()
```

## Tips for sharing the challenge online

Using [ngrok](https://ngrok.com/), you can share the challenge online:

```
ngrok http 4000
```

It will generate a link on which the challenge will be hosted publicly.
