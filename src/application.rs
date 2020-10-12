use crate::bmp::*;
use crate::configuration::*;
use crate::dpad::*;
use crate::gambatte::*;
use crate::gfx::*;
use crate::key::*;
use crate::platform::*;
use crate::winapi::*;

use std::thread;
use std::time::*;

pub const KEY_SIZE: i32 = 34;
pub const ARROW_SIZE: i32 = 14;
pub const CHARACTER_SIZE: i32 = 16;

pub const WIDTH: i32 = KEY_SIZE * 9;
pub const HEIGHT: i32 = KEY_SIZE * 8;
pub const SCALE: i32 = 1;
pub const TITLE: &str = "Input Display";

pub const UP: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const RIGHT: usize = 3;
pub const POWER: usize = 8;

pub struct Application {
    pub platform: Platform,
    pub keyset: SpriteSheet,
    pub arrowset: SpriteSheet,
    pub font: SpriteSheet,
    pub palettes: Vec<(String, Palette)>,
    pub palette: Palette,
    pub palette_index: usize,
    pub keys: Vec<Key>,
    pub dpad: Vec<DpadKey>,
    pub key_to_configure: i32,
    pub text_buffer: String,
    pub gambatte_sync: bool,
}

pub static mut APP_POINTER: *mut Application = std::ptr::null_mut();

pub fn start() {
    let mut app = Application {
        platform: Platform::new(WIDTH, HEIGHT, SCALE, TITLE).unwrap(),
        keyset: SpriteSheet::new(bmp_load(include_bytes!("gfx/keys.bmp")).unwrap(), KEY_SIZE, KEY_SIZE),
        arrowset: SpriteSheet::new(bmp_load(include_bytes!("gfx/arrows.bmp")).unwrap(), ARROW_SIZE, ARROW_SIZE),
        font: SpriteSheet::new(bmp_load(include_bytes!("gfx/font.bmp")).unwrap(), CHARACTER_SIZE, CHARACTER_SIZE),
        palettes: vec![
            (String::from("Brown"), vec![[0, 0, 0], [228, 150, 133], [228, 150, 133], [248, 248, 248]]),
            (String::from("Pastel Mix"), vec![[0, 0, 0], [228, 144, 163], [228, 144, 163], [242, 226, 187]]),
            (String::from("Blue"), vec![[0, 0, 0], [225, 128, 150], [113, 182, 208], [248, 248, 248]]),
            (String::from("Green"), vec![[0, 0, 0], [96, 186, 46], [96, 186, 46], [248, 248, 248]]),
            (String::from("Red"), vec![[0, 0, 0], [131, 198, 86], [225, 128, 150], [248, 248, 248]]),
            (String::from("Orange"), vec![[0, 0, 0], [232, 186, 77], [232, 186, 77], [248, 248, 248]]),
            (String::from("Dark Blue"), vec![[0, 0, 0], [225, 128, 150], [141, 156, 191], [248, 248, 248]]),
            (String::from("Dark Green"), vec![[0, 0, 0], [225, 128, 150], [131, 198, 86], [248, 248, 248]]),
            (String::from("Dark Brown"), vec![[78, 38, 28], [228, 150, 133], [189, 146, 144], [241, 216, 206]]),
            (String::from("Yellow"), vec![[0, 0, 0], [113, 182, 208], [232, 186, 77], [248, 248, 248]]),
            (String::from("Monochrome"), vec![[0, 0, 0], [160, 160, 160], [160, 160, 160], [248, 248, 248]]),
            (String::from("Inverted"), vec![[248, 248, 248], [24, 128, 104], [24, 128, 104], [0, 0, 0]]),
        ],
        palette: Vec::new(),
        palette_index: 0,
        keys: vec![
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("UP"), reg_entry: String::from("GameUpKey"), x: 2.0, y: 2.0, idx: 5 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("DOWN"), reg_entry: String::from("GameDownKey"), x: 2.0, y: 4.0, idx: 6 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("LEFT"), reg_entry: String::from("GameLeftKey"), x: 1.0, y: 3.0, idx: 7 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("RIGHT"), reg_entry: String::from("GameRightKey"), x: 3.0, y: 3.0, idx: 8 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("SELECT"), reg_entry: String::from("GameSelectKey"), x: 3.5, y: 6.0, idx: 2 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("START"), reg_entry: String::from("GameStartKey"), x: 4.5, y: 6.0, idx: 3 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("B"), reg_entry: String::from("GameBKey"), x: 5.5, y: 4.0, idx: 1 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("A"), reg_entry: String::from("GameAKey"), x: 7.0, y: 3.0, idx: 0 },
            Key { primary_ipt: 0, secondary_ipt: 0, name: String::from("POWER"), reg_entry: String::from("PlayHard resetKey"), x: 7.0, y: 1.0 - (6.0 / KEY_SIZE as f32), idx: 4 },
        ],
        dpad: vec![
            // TODO: Is there a better way to construct overhangs?
            DpadKey::new(UP, 0, -2, 2, KEY_SIZE - 2, 2, KEY_SIZE, KEY_SIZE - 4, 2, 10, 8),
            DpadKey::new(DOWN, 0, 2, 2, 0, 2, -2, KEY_SIZE - 4, 2, 10, 6),
            DpadKey::new(LEFT, -2, 0, KEY_SIZE - 2, 2, KEY_SIZE, 2, 2, KEY_SIZE - 4, 8, 7),
            DpadKey::new(RIGHT, 2, 0, 0, 2, -2, 2, 2, KEY_SIZE - 4, 12, 7),
        ],
        key_to_configure: -1,
        text_buffer: String::from(""),
        gambatte_sync: false,
    };

    app.platform.register_callback(WM_CLOSE, on_quit);
    app.platform.register_callback(WM_RBUTTONUP, on_rightclick);
    app.platform.register_hook(WH_KEYBOARD_LL, on_key_state);

    unsafe { APP_POINTER = &mut app };

    match load_configuration() {
        Ok(_) => {}
        Err(_) => change_palette(3),
    }

    app.platform.start_message_queue();
}

fn on_quit(_wparam: usize, _lparam: usize) {
    let mut app = unsafe { &mut *APP_POINTER };
    app.platform.running = false;
}

fn on_rightclick(_wparam: usize, _lparam: usize) {
    let app = unsafe { &mut *APP_POINTER };
    let mut item_counter = 0;

    let mut palette_menu: Vec<MenuItem> = Vec::new();

    for (i, pal) in app.palettes.iter().enumerate() {
        palette_menu.push(if app.palette_index == i { MenuItem::Checked(pal.0.clone()) } else { MenuItem::Unchecked(pal.0.clone()) });
    }

    let res = app.platform.show_menu(
        &[
            MenuItem::Unchecked(String::from("Sync Gambatte Keybinds")),
            MenuItem::Unchecked(String::from("Set Keybinds")),
            MenuItem::Seperator,
            MenuItem::SubMenu(String::from("Palettes"), app.platform.create_menu(palette_menu.as_slice(), &mut item_counter)),
        ],
        &mut item_counter,
    ) as usize;

    if res == palette_menu.len() + 1 {
        let result_text = match sync_gambatte_keybindings() {
            Ok(_) => "SUCCESS",
            Err(_) => "FAILURE",
        };
        thread::spawn(move || {
            app.text_buffer = String::from(result_text);
            draw_background();
            app.text_buffer = String::from("");
            thread::sleep(Duration::from_secs(3));
            draw_background();
        });
    } else if res == palette_menu.len() + 2 {
        configure_next_key();
    } else if res > 0 && res <= palette_menu.len() {
        change_palette(res as usize - 1)
    }

    save_configuration().ok();
}

pub fn draw_background() {
    let app = unsafe { &mut *APP_POINTER };

    macro_rules! coord {
        ($base:expr, $offs:expr) => {
            ($base * KEY_SIZE as f32) as i32 + $offs
        };
    }

    macro_rules! draw {
        ($sheet:expr, $x:expr, $y:expr, $idx:expr) => {
            app.platform.offscreen_buffer.draw_sprite(&app.palette, $sheet, coord!($x, 0), coord!($y, 0) as i32, $idx)
        };
    }

    update_dpad();

    app.platform.offscreen_buffer.clear(app.palette[4]);
    for key in app.keys.iter() {
        draw!(&app.keyset, key.x, key.y, key.idx);
    }

    draw!(&app.keyset, app.keys[UP].x, app.keys[UP].y + 1.0, 32);

    for dpad in app.dpad.iter() {
        let key = &app.keys[dpad.key_id];
        draw!(
            &app.arrowset,
            key.x + (dpad.arrow.key_x_offset + dpad.arrow.current_x_shift) as f32 / KEY_SIZE as f32,
            key.y + (dpad.arrow.key_y_offset + dpad.arrow.current_y_shift) as f32 / KEY_SIZE as f32,
            dpad.arrow.idx
        );

        app.platform.offscreen_buffer.draw_subsprite(
            &app.palette,
            &app.keyset,
            coord!(key.x, dpad.overhang.x_dest),
            coord!(key.y, dpad.overhang.y_dest),
            key.idx,
            dpad.overhang.x_src,
            dpad.overhang.y_src,
            dpad.overhang.width,
            dpad.overhang.height,
        );
    }

    app.platform.offscreen_buffer.draw_text(&app.palette, &app.font, &app.text_buffer, (WIDTH - app.text_buffer.len() as i32 * CHARACTER_SIZE) / 2, coord!(7.0, (KEY_SIZE - CHARACTER_SIZE) / 2));

    app.platform.update_window();
}

pub fn change_palette(index: usize) {
    let app = unsafe { &mut *APP_POINTER };
    if index >= app.palettes.len() {
        return;
    }

    let mut new_palette = app.palettes[index].1.clone();

    // 2 palette entries are inserted:
    //   The first one is reserved for the future
    //   The second one is the background color + 1 green for chromakey purposes
    new_palette.insert(3, [0, 0, 0]);
    new_palette.insert(4, [new_palette[4][0], new_palette[4][1] + 1, new_palette[4][2]]);

    app.palette = new_palette;
    app.palette_index = index;
    draw_background();
}

unsafe extern "system" fn on_key_state(code: i32, wparam: usize, lparam: usize) -> usize {
    let app = &mut *APP_POINTER;
    let key_code = std::ptr::read(lparam as *const u32);
    let key_state = app.platform.key_state_from_wparam(wparam);

    if app.key_to_configure == -1 {
        let mut key = app.keys.iter_mut().find(|key| key.primary_ipt == key_code || key.secondary_ipt == key_code);

        if let None = key {
            if (app.platform.is_key_down(VK_CONTROL) && key_code as u32 == 'R' as u32) || (app.platform.is_key_down('R' as u32) && key_code == VK_CONTROL) {
                key = Some(&mut app.keys[POWER]);
            }
        }

        if let Some(key) = key {
            key.set_pressed(key_state == KeyState::Pressed);
            draw_background();
        }
    } else if key_state == KeyState::Pressed {
        configure_current_key(key_code);
    }

    return CallNextHookEx(0, code, wparam, lparam);
}
