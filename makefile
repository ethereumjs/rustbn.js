SHELL := /bin/bash
notfound=0

all:
	cargo build --target=asmjs-unknown-emscripten --release
	mkdir -p lib
	find target/asmjs-unknown-emscripten/release -type f -name "rustbn-js.js" | xargs -I {} cp {} lib/index.asm.js
	@if [ "$$(sed -n '/run()$$/p' lib/index.asm.js | wc -l)" == "0" ]; then\
		echo "ERROR: could not find run() function in generated code";\
		exit 1;\
	fi;\

	@if [ "$$(sed -n '/process\[\"on\"\](\"uncaughtException\",(function(ex){if(!(ex instanceof ExitStatus)){throw ex}}))/p' lib/index.asm.js | wc -l)" == "0" ]; then\
		echo "ERROR: could not find emscripten code (catching all exceptions)":\
		exit 2;\
	fi; \

	sed -ibak 's/run()$$/Module\["arguments"\]=\[\];run();module\.exports=Module;/' lib/index.asm.js
	sed -ibak 's/process\["on"\]("uncaughtException",(function(ex){if(!(ex instanceof ExitStatus)){throw ex}}))/\/\*removed uncaught exception exit\*\//' lib/index.asm.js

wasm:
	cargo build --target=wasm32-unknown-emscripten --release
	mkdir -p exp
	find target/wasm32-unknown-emscripten/release/deps -type f -name "*.wasm" | xargs -I {} cp {} exp/rustbn.wasm
	find target/wasm32-unknown-emscripten/release/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} exp/index.wasm.js
