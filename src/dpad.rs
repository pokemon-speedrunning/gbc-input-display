use crate::application::*;
use crate::key::*;

pub const ARROW_PRESSED_BIT: i32 = 4;

pub struct DpadKey {
    pub key_id: usize,
    pub pressed_x_shift: i32,
    pub pressed_y_shift: i32,
    pub overhang: Overhang,
    pub arrow: Arrow,
}

pub struct Overhang {
    pub x_src: i32,
    pub y_src: i32,
    pub x_dest: i32,
    pub y_dest: i32,
    pub width: i32,
    pub height: i32,
}

pub struct Arrow {
    pub idx: i32,
    pub key_x_offset: i32,
    pub key_y_offset: i32,
    pub current_x_shift: i32,
    pub current_y_shift: i32,
}

impl DpadKey {
    pub fn new(
        idx: usize,
        pressed_x_shift: i32,
        pressed_y_shift: i32,
        overhang_x_src: i32,
        overhang_y_src: i32,
        overhang_x_dest: i32,
        overhang_y_dest: i32,
        overhang_width: i32,
        overhang_height: i32,
        arrow_x_offset: i32,
        arrow_y_offset: i32,
    ) -> Self {
        return DpadKey {
            key_id: idx,
            pressed_x_shift: pressed_x_shift,
            pressed_y_shift: pressed_y_shift,
            overhang: Overhang {
                x_src: overhang_x_src,
                y_src: overhang_y_src,
                x_dest: overhang_x_dest,
                y_dest: overhang_y_dest,
                width: overhang_width,
                height: overhang_height,
            },
            arrow: Arrow {
                idx: idx as i32,
                key_x_offset: arrow_x_offset,
                key_y_offset: arrow_y_offset,
                current_x_shift: 0,
                current_y_shift: 0,
            },
        };
    }
}

pub fn update_dpad() {
    let app = unsafe { &mut *APP_POINTER };

    let mut total_x_shift = 0;
    let mut total_y_shift = 0;

    for dpad in app.dpad.iter_mut() {
        dpad.arrow.idx &= !ARROW_PRESSED_BIT;
        let key = &mut app.keys[dpad.key_id];
        if key.is_pressed() {
            dpad.arrow.idx |= ARROW_PRESSED_BIT;
            total_x_shift += dpad.pressed_x_shift;
            total_y_shift += dpad.pressed_y_shift;
        }
    }

    let length_bits = if total_y_shift < 0 {
        KEY_ELONGATED_BIT
    } else if total_y_shift > 0 {
        KEY_CONTRACTED_BIT
    } else {
        0
    };

    for dpad in app.dpad.iter_mut() {
        dpad.arrow.current_x_shift = total_x_shift;
        dpad.arrow.current_y_shift = total_y_shift;

        let key = &mut app.keys[dpad.key_id];
        key.clear_length_state();
        if !key.is_pressed() {
            key.set_length_state(length_bits);
        }
    }
}
