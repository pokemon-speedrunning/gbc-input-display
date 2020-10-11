pub const KEY_PRESSED_BIT: i32 = 16;
pub const KEY_CONTRACTED_BIT: i32 = 32;
pub const KEY_ELONGATED_BIT: i32 = 64;

pub struct Key {
    pub ipt: u32,
    pub x: f32,
    pub y: f32,
    pub idx: i32,
    pub name: String,
    pub reg_entry: String,
}

impl Key {
    pub fn is_pressed(&self) -> bool {
        return (self.idx & KEY_PRESSED_BIT) > 0;
    }

    pub fn set_pressed(&mut self, value: bool) {
        if value {
            self.idx |= KEY_PRESSED_BIT;
        } else {
            self.idx &= !KEY_PRESSED_BIT;
        }
    }

    pub fn clear_length_state(&mut self) {
        self.idx &= !(KEY_CONTRACTED_BIT | KEY_ELONGATED_BIT);
    }

    pub fn set_length_state(&mut self, value: i32) {
        self.idx |= value;
    }
}
