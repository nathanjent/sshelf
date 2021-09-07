mod palettes;
mod sprites;
mod wasm4;

use palettes::{change_palette, set_draw_color, DUSTBYTE, W4};
use sprites::*;
use wasm4::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

type SpriteSheet<'a> = &'a [u8];

#[derive(Debug)]
struct Game<'a> {
    entities: [Entity<'a>; 10],
    tic: u64,
}

#[derive(Debug, Clone, Copy)]
struct Entity<'a> {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    vx_max: f32,
    vy_max: f32,
    ax: f32,
    ay: f32,
    sprite: Option<Sprite<'a>>,
    mode: Mode<'a>,
    cur_frame: usize,
}

#[derive(Debug, Clone, Copy)]
struct Sprite<'a> {
    sheet: SpriteSheet<'a>,
    width: u32,
    height: u32,
    src_x: u32,
    src_y: u32,
    stride: i32,
    flags: u32,
    draw_color: u16,
}

#[derive(Debug, Clone, Copy)]
enum Mode<'a> {
    Idle { frames: &'a [u8] },
    Walking { frames: &'a [u8] },
}

#[derive(Debug, Clone, Copy)]
struct Map<'a> {
    sprite_sheet: SpriteSheet<'a>,
    tile_width: u8,
    tile_height: u8,
    tile_map: &'a [u8],
    tile_flags: &'a [u16],
}

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| {
    Mutex::new(Game {
        tic: 0,
        entities: [Entity {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            vx_max: 1.3,
            vy_max: 1.3,
            ax: 0.1,
            ay: 0.1,
            sprite: None,
            mode: Mode::Idle { frames: &[0] },
            cur_frame: 0,
        }; 10],
    })
});

#[no_mangle]
fn start() {
    change_palette(DUSTBYTE);
    // Tip: Palette order is determined by first seen when scanning from top left by row
    let mut game = GAME.lock().unwrap();

    let player = &mut game.entities[0];
    player.x = 64.;
    player.y = 64.;
    player.sprite = Some(Sprite {
        width: 16,
        height: 16,
        flags: SPRITESHEETFLAGS,
        sheet: &SPRITESHEET,
        src_x: 0,
        src_y: 0,
        stride: SPRITESHEETWIDTH as i32,
        draw_color: 0x4320u16,
    });

    let knight = &mut game.entities[1];
    knight.x = 30.;
    knight.y = 10.;
    knight.sprite = Some(Sprite {
        width: 16,
        height: 16,
        flags: SPRITESHEETFLAGS,
        sheet: &SPRITESHEET,
        src_x: 0,
        src_y: 16,
        stride: SPRITESHEETWIDTH as i32,
        draw_color: 0x4320u16,
    });
}

#[no_mangle]
fn update() {
    input();
    draw();
}

fn input() {
    let mut game = GAME.lock().unwrap();
    game.tic += 1;
    let gamepad = unsafe { *GAMEPAD1 };
    let player = &mut game.entities[0];
    if gamepad & BUTTON_1 != 0 {}
    if gamepad & BUTTON_2 != 0 {}
    if gamepad & BUTTON_LEFT != 0 && player.vx > -player.vx_max {
        player.vx -= player.ax;
        if let Some(ref mut sprite) = player.sprite {
            sprite.flags |= BLIT_FLIP_X;
        }
    } else if gamepad & BUTTON_RIGHT != 0 && player.vx < player.vx_max {
        player.vx += player.ax;
        if let Some(ref mut sprite) = player.sprite {
            sprite.flags &= !BLIT_FLIP_X;
        }
    } else {
        if player.vx > 0.3 {
            player.vx -= player.ax;
        } else if player.vx < -0.3 {
            player.vx += player.ax;
        } else {
            player.vx = 0.0;
        }
    }

    if gamepad & BUTTON_UP != 0 && player.vy > -player.vy_max {
        player.vy -= player.ay;
    } else if gamepad & BUTTON_DOWN != 0 && player.vy < player.vy_max {
        player.vy += player.ay;
    } else {
        if player.vy > 0.3 {
            player.vy -= player.ay;
        } else if player.vy < -0.3 {
            player.vy += player.ay;
        } else {
            player.vy = 0.0;
        }
    }
    player.x += player.vx;
    player.y += player.vy;
}

fn draw() {
    let game = GAME.lock().unwrap();
    for entity in game.entities.iter().rev() {
        if let Some(sprite) = &entity.sprite {
            set_draw_color(sprite.draw_color);
            blit_sub(
                &sprite.sheet,
                entity.x as i32,
                entity.y as i32,
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
