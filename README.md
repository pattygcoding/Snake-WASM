# Snake-WASM

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Macroquad](https://img.shields.io/badge/Macroquad-0.4-black)](https://macroquad.rs/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-wasm32-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![HTML5 Canvas](https://img.shields.io/badge/HTML5-Canvas-e34f26?logo=html5&logoColor=white)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)

A small Rust + Macroquad Snake game for a portfolio page, with smooth movement,
rounded procedural graphics, and keyboard and mouse controls. Graphics are drawn
in code. The bundled Outfit font is compiled into the WASM with `include_bytes!`,
so no image, texture, or font downloads are needed at runtime.

Rendering interpolates between grid positions without changing the 0.12-second
movement tick. Game phases are explicit: ready, playing, paused, game over, and won.

## Play

- Space or Enter: start.
- Arrow keys or WASD: steer.
- P: pause or resume.
- Space: restart at any time.
- Click Play, Resume, or Play again on the corresponding screen.
- Use the pause/resume and restart icons above the board; hover for tooltips.

Eat red apples to grow and score. Walls and self-bites end the round. Filling
the 20 x 20 board wins. The best score lasts until the page is reloaded.
Only one turn is accepted per movement tick, preventing rapid-key reversals.
The board scales to fit the canvas without stretching; input requires a keyboard.

## Minimal Cargo.toml

```toml
[package]
name = "snake-wasm"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```

## Build

With Rust installed, run these two commands from the project directory:

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Output: `target/wasm32-unknown-unknown/release/snake-wasm.wasm`.
Keep Cargo.lock in version control to retain the tested dependency versions.
No wasm-bindgen, wasm-pack, or native graphics SDK is required for this target.

## Development Checks

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The tests cover movement timing, reversal protection, growth, scoring, collisions,
the vacating-tail case, pause/resume, restart, full-board wins, and responsive
button hit areas. Native tests require the platform's Rust linker/toolchain.

## Host on Your Portfolio

A Macroquad WASM binary needs an HTML canvas and Macroquad's JavaScript runtime
loader; uploading the binary alone does not display the game.
[index.html](index.html) is a minimal host, not a second game implementation.

1. Place the built `snake-wasm.wasm` next to `index.html` on your static host.
2. Serve the files over HTTP or HTTPS, not `file://`. Configure the server to
	 send `.wasm` files as `application/wasm`.
3. Open the page directly, or embed it in your portfolio with an iframe:

```html
<iframe
	src="/snake/index.html"
	title="Play Snake"
	style="width: 100%; max-width: 600px; height: 620px; border: 0"
></iframe>
```

The host currently loads `mq_js_bundle.js` from Macroquad's official samples
site. For a self-contained production deployment, download the tested loader,
host it beside the HTML and WASM, and change the script URL to
`./mq_js_bundle.js`. Keep the loader compatible with the locked Macroquad version.
It is runtime glue, not an image, texture, font, or gameplay asset.

For a local preview, put the built WASM beside the HTML and serve the project
directory with any static HTTP server. Click the game to focus it when embedded.

## Font License

[Outfit](https://github.com/Outfitio/Outfit-Fonts) is distributed under the
[SIL Open Font License](assets/fonts/OFL.txt). The bundled file is Outfit Medium
from the upstream Outfit-Fonts repository. Include this license with distributions of the game,
including a copy alongside the WASM when deploying to your portfolio.