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

target/wasm32-unknown-unknown/release/cart.wasm: src/sprites.rs
	cargo build --release

all: target/wasm32-unknown-unknown/release/cart.wasm

.PHONY: clean
clean:
	cargo clean
	rm -f src/sprites.rs
	rm -rf sprites/
