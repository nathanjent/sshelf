# Convert sprite assets to code constants and build cart
all: build/cart.wasm

IMGPATH := assets
IMAGES_TO_CONVERT := $(sort $(wildcard $(IMGPATH)/*.png))
$(info Images to convert to indexed: $(IMAGES_TO_CONVERT))

BUILD_DIR := build
SPRITESHEET := $(BUILD_DIR)/spritesheet.png
SPRITES_CODE := src/sprites.rs

# Output cart to location expected by `w4 watch` when a Makefile is present
CART := $(BUILD_DIR)/cart.wasm

$(SPRITE_CODE): $(IMAGES_TO_CONVERT)
	@echo "Combine images into single spritesheet: $< -> $(SPRITESHEET)"
	convert $^ \
		-define png:color-type=3 \
		-define png:bit-depth=4 \
		-define png:exclude-chunks=all \
		-append $(SPRITESHEET)
	@echo "Convert to 4 color indexed PNG: $< -> $@"
	echo "/// Sprite data" > $@
	@echo "Convert sprites to Rust code: $^ -> $@"
	w4 png2src --rs $(SPRITESHEET) >> $@
	# Uppercase variable names
	sed -i 's/^const \(.*\):/pub const \U\1:/g' $@

$(CART): $(SPRITE_CODE) src/*.rs
	cargo build --release
	mkdir -p build
	cp target/wasm32-unknown-unknown/release/cart.wasm $(CART)

.PHONY: clean run
run: $(CART)
	w4 run $(CART)

clean:
	cargo clean
	rm -f $(SPRITE_OUT)
	rm -rf $(SPRITESHEET)
	rm -f $(CART)
