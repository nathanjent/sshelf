src/sprites.rs:
	mkdir -p sprites
	find assets/ -type f -name '*.png' -print0 | \
		while IFS='' read -r -d '' file; do \
			convert $$file \
				-define png:color-type=3 \
				-define png:bit-depth=4 \
				-define png:exclude-chunks=all \
				sprites/$$(basename $$file); \
		done
	rm -f src/sprites.rs
	find sprites/ -name '*.png' -exec w4 png2src --rs '{}' >> src/sprites.rs +
	sed -i 's/^const \(.*\):/pub const \U\1:/g' src/sprites.rs

build/cart.wasm: src/sprites.rs src/*.rs
	cargo build --release
	# Output to location expected by `w4 watch` when a Makefile is present
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/cart.wasm build/cart.wasm

all: build/cart.wasm

.PHONY: clean
clean:
	cargo clean
	rm -f src/sprites.rs
	rm -rf sprites/
