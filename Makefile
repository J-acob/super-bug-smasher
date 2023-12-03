wasm_bind:
	wasm-bindgen --no-typescript --out-dir wasm --target web target/wasm32-unknown-unknown/release/bug-game.wasm

monkey_patch:
	sed -i.bak \
		-e 's/getObject(arg0).fetch(/window.bevyLoadingBarFetch(/' \
		-e 's/input = fetch(/input = window.bevyLoadingBarFetch(/' \
		wasm/bug-game.js

distrib_web: wasm_bind monkey_patch