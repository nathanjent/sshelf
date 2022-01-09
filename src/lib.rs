#[cfg(feature = "buddy-alloc")]
mod alloc;
mod palettes;
mod sprites;
mod wasm4;
mod game;

use std::cell::Cell;
use palettes::{change_palette, set_draw_color, DUSTBYTE, W4};
use wasm4::*;
use game::Entity;

const ENTITY_COUNT: usize = 3;

//#[derive(Debug, Clone, Copy)]
//enum Mode<'a> {
//    Idle { frames: &'a [u8] },
//    Walking { frames: &'a [u8] },
//}

//#[derive(Debug, Clone, Copy)]
//struct Map<'a> {
//    sprite_sheet: SpriteSheet<'a>,
//    tile_width: u8,
//    tile_height: u8,
//    tile_map: &'a [u8],
//    tile_flags: &'a [u16],
//}

const PLAYER_ID: Cell<usize> = Cell::new(0);
const TICK: Cell<u64> = Cell::new(0);

#[no_mangle]
fn start() {
    // Tip: Palette order is determined by first seen when scanning from top left by row
    change_palette(DUSTBYTE);

    if let Some(player) = game::get_entity(PLAYER_ID.get()) {
        game::set_entity(Entity {
            x: 64.0,
            y: 64.0,
            vx_max: 1.3,
            vy_max: 1.3,
            ax: 0.1,
            ay: 0.1,
            sprite: Some("elf"),
            ..player
        });
    }


    for i in 1..ENTITY_COUNT {
        if let Some(entity) = game::get_entity(i) {
            game::set_entity(Entity {
                x: 64.0,
                y: 64.0,
                vx_max: 1.3,
                vy_max: 1.3,
                ax: 0.1,
                ay: 0.1,
                sprite: Some("knight"),
                ..entity
            });
        }
    }
}

#[no_mangle]
fn update() {
    input();
    draw();
}

fn input() {
    TICK.set(TICK.get() + 1);
    if let Some(mut player) = game::get_entity(PLAYER_ID.get()) {
        let gamepad = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_1 != 0 {}
        if gamepad & BUTTON_2 != 0 {}
        if gamepad & BUTTON_LEFT != 0 && player.vx > -player.vx_max {
            player.vx -= player.ax;
            player.flip = true;
        } else if gamepad & BUTTON_RIGHT != 0 && player.vx < player.vx_max {
            player.vx += player.ax;
            player.flip = true;
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

        // Update the entity
        game::set_entity(player);
    }
}

fn draw() {
    for i in 0..ENTITY_COUNT {
        if let Some(entity) = game::get_entity(i) {
            if let Some(sprite_name) = &entity.sprite {
                if let Some(sprite) = sprites::get_sprite(&sprite_name) {
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
                        }
                    );
                }
            }
        }
    }
}
