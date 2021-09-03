mod wasm4;
use wasm4::*;

mod sprites;
use sprites::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

type Sprite = [u8; 32];

#[derive(Debug, Default)]
struct Game {
    entities: [Entity; 10],
}

#[derive(Debug, Default, Clone, Copy)]
struct Entity {
    x: i32,
    y: i32,
    sprite: Sprite,
}

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| Mutex::new(Game {
    entities: [Entity {
        x: 0,
        y: 0,
        sprite: ELF,
    };10],
}));

#[no_mangle]
fn start() {
    let mut palette = unsafe { *PALETTE };
    palette[0]= 0x071821;
    palette[1]= 0x306850;
    palette[2]= 0x86c06c;
    palette[3]= 0xe0f8cf;

    let mut game = GAME.lock().unwrap();

    let player = &mut game.entities[0];
    player.x = 64;
    player.y = 64;

    let monster = &mut game.entities[1];
    monster.x = 30;
    monster.y = 10;
}

#[no_mangle]
fn update () {
    input();
    draw();
}

fn input() {
    let mut game = GAME.lock().unwrap();
    let gamepad = unsafe { *GAMEPAD1 };
    let player = &mut game.entities[0];
    if gamepad & BUTTON_1 != 0 {
    }
    if gamepad & BUTTON_2 != 0 {
    }
    if gamepad & BUTTON_LEFT != 0 {
        player.x -= 1;
    } else if gamepad & BUTTON_RIGHT != 0 {
        player.x += 1;
    }
    if gamepad & BUTTON_UP != 0 {
        player.y -= 1;
    }
    if gamepad & BUTTON_DOWN != 0 {
        player.y += 1;
    }
}

fn draw() {
    unsafe { *DRAW_COLORS = 0x3421 }
    let game = GAME.lock().unwrap();
    for entity in &game.entities {
        blit_sub(&entity.sprite, entity.x, entity.y, 8, ELFHEIGHT, 8, 0, ELFWIDTH as i32, ELFFLAGS);
    }
}
