# Cup Stack

3D cup-stacking game built with [Bevy](https://bevyengine.org) and [Rapier](https://rapier.rs), compiled to WebAssembly. Extracted from [gotcha](https://github.com/tcerqueira/gotcha).

## Run

```sh
cargo run
```

In the browser, with the `wasm32-unknown-unknown` target and [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) installed:

```sh
cargo run --target wasm32-unknown-unknown
```

## Bundle for web

Requires the [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli). Uses `web/index.html` as the page shell.

```sh
bevy build --yes --profile=wasm-release web --bundle
```

Output goes to `target/bevy_web/wasm-release/cup-stack/`.
