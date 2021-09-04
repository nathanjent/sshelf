mod palettes;
mod sprites;
mod wasm4;

use palettes::{change_palette, set_draw_color, DUSTBYTE, W4};
use sprites::*;
use wasm4::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

type SpriteSheet = [u8; 128];

#[derive(Debug)]
struct Game {
    entities: [Entity; 10],
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    x: i32,
    y: i32,
    sprite: Option<Sprite>,
}

#[derive(Debug, Clone, Copy)]
struct Sprite {
    sheet: SpriteSheet,
    width: u32,
    height: u32,
    src_x: u32,
    src_y: u32,
    stride: i32,
    flags: u32,
}

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| {
    Mutex::new(Game {
        entities: [Entity {
            x: 0,
            y: 0,
            sprite: None,
        }; 10],
    })
});

#[no_mangle]
fn start() {
    change_palette(DUSTBYTE);
    // Tip: Palette order is determined by first seen when scanning from top left by row
    set_draw_color(0x4320u16);

    let mut game = GAME.lock().unwrap();

    let player = &mut game.entities[0];
    player.x = 64;
    player.y = 64;
    player.sprite = Some(Sprite {
        width: 16,
        height: 16,
        flags: SPRITESHEETFLAGS,
        sheet: SPRITESHEET,
        src_x: 0,
        src_y: 0,
        stride: SPRITESHEETWIDTH as i32,
    });

    let knight = &mut game.entities[1];
    knight.x = 30;
    knight.y = 10;
    knight.sprite = Some(Sprite {
        width: 16,
        height: 16,
        flags: SPRITESHEETFLAGS,
        sheet: SPRITESHEET,
        src_x: 0,
        src_y: 16,
        stride: SPRITESHEETWIDTH as i32,
    });
}

#[no_mangle]
fn update() {
    input();
    draw();
}

fn input() {
    let mut game = GAME.lock().unwrap();
    let gamepad = unsafe { *GAMEPAD1 };
    let player = &mut game.entities[0];
    if gamepad & BUTTON_1 != 0 {}
    if gamepad & BUTTON_2 != 0 {}
    if gamepad & BUTTON_LEFT != 0 {
        player.x -= 1;
        if let Some(mut sprite) = player.sprite {
            // FIXME sprite not flipping correctly
            sprite.flags |= BLIT_FLIP_X;
            trace_fmt(&format_args!("flip left: {:b}", sprite.flags), 64, 64);
        }
    } else if gamepad & BUTTON_RIGHT != 0 {
        player.x += 1;
        if let Some(mut sprite) = player.sprite {
            sprite.flags &= !BLIT_FLIP_X;
            trace_fmt(&format_args!("flip right: {:b}", sprite.flags), 64, 64);
        }
    }
    if gamepad & BUTTON_UP != 0 {
        player.y -= 1;
    }
    if gamepad & BUTTON_DOWN != 0 {
        player.y += 1;
    }
}

fn draw() {
    let game = GAME.lock().unwrap();

    for entity in game.entities.iter().rev() {
        if let Some(sprite) = &entity.sprite {
            blit_sub(
                &sprite.sheet,
                entity.x,
                entity.y,
                sprite.width,
                sprite.height,
                sprite.src_x,
                sprite.src_y,
                sprite.stride,
                sprite.flags,
            );
        }
    }
}

fn trace_fmt(fmt_args: &std::fmt::Arguments, x: i32, y: i32) {
    if let Some(out) = fmt_args.as_str() {
        trace(&out, x, y);
    }
}
