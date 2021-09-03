# Convert sprite assets to code constants and build cart
all: build/cart.wasm

IMGPATH := assets
IMAGES_TO_CONVERT := $(sort $(wildcard $(IMGPATH)/*.png))
$(info Images to convert to indexed: $(IMAGES_TO_CONVERT))

SPRTPATH := sprites

src/sprites.rs: $(IMAGES_TO_CONVERT)
	@echo "Convert to 4 color indexed PNG: $< -> $@"
	mkdir -p $(SPRTPATH)
	@echo "Convert sprites to Rust code: $^ -> $@"
	echo "/// Sprite data" > src/sprites.rs
	for img in $^; do \
		convert $$img \
			-define png:color-type=3 \
			-define png:bit-depth=4 \
			-define png:exclude-chunks=all \
			$(SPRTPATH)/`basename $$img`; \
		echo ""; \
		w4 png2src --rs $(SPRTPATH)/`basename $$img`; \
	done >> $@
	# Uppercase variable names
	sed -i 's/^const \(.*\):/pub const \U\1:/g' src/sprites.rs

build/cart.wasm: src/sprites.rs src/*.rs
	cargo build --release
	# Output to location expected by `w4 watch` when a Makefile is present
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/cart.wasm build/cart.wasm

.PHONY: clean
clean:
	cargo clean
	rm -f src/sprites.rs
	rm -rf sprites/
