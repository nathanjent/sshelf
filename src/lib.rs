mod wasm4;
use wasm4::*;

mod sprites;
use sprites::*;

#[no_mangle]
fn update () {
    let mut palette = unsafe { *PALETTE };
    palette[0]= 0x071821;
    palette[1]= 0x306850;
    palette[2]= 0x86c06c;
    palette[3]= 0xe0f8cf;

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
    }

    unsafe { *DRAW_COLORS = 0x3421 }
    blit_sub(&ELF, 76, 76, 8, ELFHEIGHT, 8, 0, ELFWIDTH as i32, ELFFLAGS);
}
