wasm-server: wasm-build
	python -m http.server

wasm-build:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_game_of_life.wasm

run-local:
	cargo run --release

build-local:
	cargo build --release
	
deploy: wasm-build
	git push -u origin main
