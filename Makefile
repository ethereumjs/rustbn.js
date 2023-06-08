SHELL := /bin/bash

all:
	cargo build --target=asmjs-unknown-emscripten --release
	mkdir -p lib
	find target/asmjs-unknown-emscripten/release -type f -name "rustbn-js.js" | xargs -I {} cp {} lib/index.asm.js
	@res=$$(sed -n '/run();$$/p' lib/index.asm.js | wc -l); \
	if [ $$res == "0" ]; then \
		echo "ERROR: could not find run() function in generated code"; \
		exit 1; \
	fi\

	sed -ibak 's/run();$$/Module\["arguments"\]=\[\];run();module\.exports=Module;/' lib/index.asm.js

wasm:
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/rustbn-js.wasm lib/rustbn.wasm
