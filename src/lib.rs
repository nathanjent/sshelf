mod palettes;
mod sprites;
mod wasm4;

use palettes::{change_palette, set_draw_color, DUSTBYTE, W4};
use sprites::*;
use wasm4::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

type SpriteSheet = [u8; 64];

#[derive(Debug)]
struct Game {
    entities: [Entity; 10],
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    x: i32,
    y: i32,
    sprite_sheet: Option<SpriteSheet>,
    sprite_width: u32,
    sprite_height: u32,
    sprite_stride: i32,
    sprite_flags: u32,
}

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| {
    Mutex::new(Game {
        entities: [Entity {
            x: 0,
            y: 0,
            sprite_sheet: None,
            sprite_width: 1,
            sprite_height: 1,
            sprite_stride: 0,
            sprite_flags: 0,
        }; 10],
    })
});

#[no_mangle]
fn start() {
    change_palette(DUSTBYTE);

    set_draw_color(0x4320u16);

    let mut game = GAME.lock().unwrap();

    let player = &mut game.entities[0];
    player.x = 64;
    player.y = 64;
    player.sprite_width = ELFWIDTH;
    player.sprite_height = ELFHEIGHT;
    player.sprite_flags = ELFFLAGS;
    player.sprite_sheet = Some(ELF);

    let knight = &mut game.entities[1];
    knight.x = 30;
    knight.y = 10;
    knight.sprite_width = KNIGHTWIDTH;
    knight.sprite_height = KNIGHTHEIGHT;
    knight.sprite_flags = KNIGHTFLAGS;
    knight.sprite_sheet = Some(KNIGHT);
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
        player.sprite_flags |= BLIT_FLIP_X
    } else if gamepad & BUTTON_RIGHT != 0 {
        player.x += 1;
        player.sprite_flags &= !BLIT_FLIP_X
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
        if let Some(sprite_sheet) = &entity.sprite_sheet {
            blit_sub(
                sprite_sheet,
                entity.x,
                entity.y,
                16,
                entity.sprite_height,
                0,
                0,
                entity.sprite_width as i32,
                entity.sprite_flags,
            );
        }
    }
}
