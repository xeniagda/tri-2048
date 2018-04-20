build:
	cargo +nightly build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/tri_2048.wasm site

debug:
	cargo +nightly build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/tri_2048.wasm site

run:
	open site/index.html