mod wasm4;
use wasm4::*;

mod sprites;
use sprites::*;

#[no_mangle]
fn update () {
    let mut palette = unsafe { *PALETTE };
    palette[0]= 0x788374;
    palette[1]= 0xf5e9bf;
    palette[2]= 0xaa644d;
    palette[3]= 0x372a39;

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
    }

    unsafe { *DRAW_COLORS = 0x1230 }
    blit(&ELF, 76, 76, ELFWIDTH, ELFHEIGHT, ELFFLAGS);
}
