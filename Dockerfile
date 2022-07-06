FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=myuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /myuser

COPY ./ ./

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli basic-http-server
RUN cargo build --target wasm32-unknown-unknown --release
RUN wasm-bindgen ./target/wasm32-unknown-unknown/release/chiphuit.wasm --out-dir build --no-typescript --target no-modules --remove-name-section  --remove-producers-section --omit-default-module-path --omit-imports

EXPOSE 4000

CMD ["basic-http-server", "build", "--addr", "0.0.0.0:4000"]
