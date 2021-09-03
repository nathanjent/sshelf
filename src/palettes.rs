use super::wasm4::{DRAW_COLORS, PALETTE};

type Palette = [u32; 4];

pub const W4: Palette = [0x071821, 0x306850, 0x86c06c, 0xe0f8cf];

// https://lospec.com/palette-list/dustbyte
pub const DUSTBYTE: Palette = [0x372a39, 0x788374, 0xaa644d, 0xf5e9bf];

pub fn change_palette(palette: Palette) {
    unsafe {
        *PALETTE = palette;
    }
}

pub fn set_draw_color<T: Into<u16>>(idx: T) {
    unsafe { *DRAW_COLORS = idx.into() }
}
