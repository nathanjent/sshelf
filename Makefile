debug ?=

$(info debug is $(debug))

# Run debug build with `make <target> debug=1`
ifdef debug
  RELEASE :=
  TARGET :=debug
else
  RELEASE :=--release
  TARGET :=release
endif

# Convert sprite assets to code constants and build cart
IMGPATH := assets
IMAGES_TO_CONVERT := $(sort $(wildcard $(IMGPATH)/*.png))
$(info Images to convert to indexed: $(IMAGES_TO_CONVERT))

BUILD_DIR := build
SPRITESHEET := $(BUILD_DIR)/spritesheet.png
SPRITES_CODE := src/sprites.rs
CARGO_OUT_DIR := target/wasm32-unknown-unknown/$(TARGET)

# Output cart to location expected by `w4 watch` when a Makefile is present
CART := $(BUILD_DIR)/cart.wasm

all: $(CART)

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

$(CARGO_OUT_DIR)/cart.wasm: $(SPRITE_CODE) src/*.rs
	cargo build $(RELEASE)

$(BUILD_DIR):
	mkdir -p build

$(CART): $(BUILD_DIR) $(CARGO_OUT_DIR)/cart.wasm
	wasm-opt -Oz -o $@ $(CARGO_OUT_DIR)/cart.wasm

.PHONY: clean run bundle
run: $(CART)
	w4 run $<

bundle: $(CART)
	@echo "Bundling cart: $<"
	w4 bundle $< \
		--linux $(BUILD_DIR)/sshelf \
		--windows $(BUILD_DIR)/sshelf.exe \
		--html $(BUILD_DIR)/sshelf.html \
		--title "Shh Elf"
	@echo "Bundling complete."

clean:
	cargo clean
	rm -rf $(BUILD_DIR)
