# Convert sprite assets to code constants and build cart

IMGPATH := assets
IMAGES_TO_CONVERT := $(wildcard $(IMGPATH)/*.png)
$(info Images to convert to indexed: $(IMAGES_TO_CONVERT))

SPRTPATH := sprites
SPRITE_IMAGES := $(sort $(addprefix sprites/,$(notdir $(IMAGES_TO_CONVERT))))
$(info Sprite Images to process into code: $(SPRITE_IMAGES))

$(SPRITE_IMAGES): $(IMAGES_TO_CONVERT)
	@echo "$< -> $@"
	mkdir -p $(SPRTPATH)
	convert $< \
		-define png:color-type=3 \
		-define png:bit-depth=4 \
		-define png:exclude-chunks=all \
		$@

src/sprites.rs: $(SPRITE_IMAGES)
	rm -f src/sprites.rs
	w4 png2src --rs $< >> src/sprites.rs
	sed -i 's/^const \(.*\):/pub const \U\1:/g' src/sprites.rs
	rm -rf $(SPRTPATH)

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
