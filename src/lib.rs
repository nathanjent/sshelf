// NOTE Any use of std::fmt such as format strings and derive Debug
// will increase the binary size by the WASM-4 cart limit

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod palettes;
mod sprites;
mod wasm4;

use palettes::{change_palette, set_draw_color, DUSTBYTE};
use std::cell::{Cell, RefCell};
use wasm4::*;

// TODO animation frames
//#[derive(Debug, Clone, Copy)]
//enum Mode<'a> {
//    Idle { frames: &'a [u8] },
//    Walking { frames: &'a [u8] },
//}

// TODO map
//#[derive(Debug, Clone, Copy)]
//struct Map<'a> {
//    sprite_sheet: SpriteSheet<'a>,
//    tile_width: u8,
//    tile_height: u8,
//    tile_map: &'a [u8],
//    tile_flags: &'a [u16],
//}

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub max_vx: f32,
    pub max_vy: f32,
    pub ax: f32,
    pub ay: f32,
    pub sprite: Option<usize>,
    pub flip: bool,
}

const PLAYER_ID: Cell<usize> = Cell::new(0);
const TICK: Cell<u64> = Cell::new(0);
const ENTITY_COUNT: usize = 3;

thread_local! {
    static ENTITIES: RefCell<[Entity; ENTITY_COUNT]> = RefCell::new([Default::default(); ENTITY_COUNT]);
}

/// Called by WASM-4 to initialize the game state
#[no_mangle]
fn start() {
    // Tip: Palette order is determined by first seen when scanning from top left by row
    change_palette(DUSTBYTE);

    ENTITIES.with(|entities| {
        let mut entities = entities.borrow_mut();

        // Init player state
        if let Some(player) = entities.get_mut(PLAYER_ID.get()) {
            player.x = 64.0;
            player.y = 64.0;
            player.max_vx = 1.3;
            player.max_vy = 1.3;
            player.ax = 0.1;
            player.ay = 0.1;
            player.sprite = Some(0);
        }

        // Init other entity states
        for id in 1..ENTITY_COUNT {
            if let Some(entity) = entities.get_mut(id) {
                entity.x = 10.0 + id as f32 * 16.0;
                entity.y = 64.0;
                entity.max_vx = 1.3;
                entity.max_vy = 1.3;
                entity.ax = 0.1;
                entity.ay = 0.1;
                entity.sprite = Some(1);
            }
        }
    });
}

/// Called by WASM-4 to update the game state
#[no_mangle]
fn update() {
    input();
    draw();
}

/// Handle controller input
fn input() {
    TICK.set(TICK.get() + 1);

    ENTITIES.with(|entities| {
        let mut entities = entities.borrow_mut();
        if let Some(player) = entities.get_mut(PLAYER_ID.get()) {
            let gamepad = unsafe { *GAMEPAD1 };

            if gamepad & BUTTON_1 != 0 {}

            if gamepad & BUTTON_2 != 0 {}

            if gamepad & BUTTON_LEFT != 0 && player.vx > -player.max_vx {
                player.vx -= player.ax;
                player.flip = true;
            } else if gamepad & BUTTON_RIGHT != 0 && player.vx < player.max_vx {
                player.vx += player.ax;
                player.flip = false;
            } else {
                if player.vx > 0.3 {
                    player.vx -= player.ax;
                } else if player.vx < -0.3 {
                    player.vx += player.ax;
                } else {
                    player.vx = 0.0;
                }
            }

            if gamepad & BUTTON_UP != 0 && player.vy > -player.max_vy {
                player.vy -= player.ay;
            } else if gamepad & BUTTON_DOWN != 0 && player.vy < player.max_vy {
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
    });
}

/// Update the display
fn draw() {
    ENTITIES.with(|entities| {
        for id in 0..ENTITY_COUNT {
            let entities = entities.borrow();
            if let Some(entity) = entities.get(id) {
                //trace(format!("x: {:?}, y: {:?}", entity.x, entity.y));
                if let Some(sprite_id) = entity.sprite {
                    if let Some(sprite) = sprites::get_sprite(sprite_id) {
                        set_draw_color(sprite.draw_color);
                        blit_sub(
                            sprite.sheet,
                            entity.x as i32,
                            entity.y as i32,
                            sprite.width,
                            sprite.height,
                            sprite.src_x,
                            sprite.src_y,
                            sprite.stride,
                            if entity.flip {
                                sprite.flags | BLIT_FLIP_X
                            } else {
                                sprite.flags & !BLIT_FLIP_X
                            },
                        );
                    }
                }
            }
        }
    });
}
