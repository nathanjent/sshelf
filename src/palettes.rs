use super::wasm4::{DRAW_COLORS, PALETTE};

type Palette = [u32; 4];

/// Default WASM-4 palette.
pub const W4: Palette = [0x071821, 0x306850, 0x86c06c, 0xe0f8cf];

/// Palette from https://lospec.com/palette-list/dustbyte
pub const DUSTBYTE: Palette = [0x372a39, 0x788374, 0xaa644d, 0xf5e9bf];

/// Change the game palette to the given palette.
pub fn change_palette(palette: Palette) {
    unsafe {
        *PALETTE = palette;
    }
}

/// Change the color being drawn using a 16 bit value.
/// 0x4321 sets the palette color used in any drawing functions.
/// 0x0000 sets all colors to transparent.
/// Drawing functions might only use some of the values.
/// The rect function uses the first color for fill and the
/// second color for outline.
pub fn set_draw_color<T: Into<u16>>(idx: T) {
    unsafe { *DRAW_COLORS = idx.into() }
}
