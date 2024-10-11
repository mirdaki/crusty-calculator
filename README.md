# Universal Calc

## Install

- [Rustup](https://rustup.rs/)
- [Node](https://nodejs.org/en/download/package-manager/)
```bash
cargo install wasm-pack
cargo install miniserve
```

## Build calc

```bash
cargo build
wasm-pack build calc --out-dir ../target/pkg-node --target nodejs
wasm-pack build calc --out-dir ../target/pkg-web --target web

cd calc-node
npm install
npx tsc
cd ../
```

## Run

```bash
cargo run -p calc-cli
cargo run -p calc-server
node ./calc-node/dist/index.js
miniserve . --index "calc-web/index.html" -p 8080
```
