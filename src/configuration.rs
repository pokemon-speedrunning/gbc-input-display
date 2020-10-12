use crate::application::*;
use crate::gambatte::*;
use crate::winapi::*;

pub fn configure_current_key(key_code: u32) {
    let app = unsafe { &mut *APP_POINTER };
    app.keys[app.key_to_configure as usize].primary_ipt = key_code;
    configure_next_key();
}

pub fn configure_next_key() {
    let app = unsafe { &mut *APP_POINTER };

    if app.key_to_configure != -1 {
        app.keys[app.key_to_configure as usize].set_pressed(false);
    }

    app.key_to_configure += 1;

    if app.key_to_configure >= app.keys.len() as i32 {
        app.key_to_configure = -1;
        app.text_buffer = String::from("");
        app.gambatte_sync = false;
        save_configuration().ok();
    } else {
        app.keys[app.key_to_configure as usize].set_pressed(true);
        app.text_buffer = String::from(format!("PRESS {}", &app.keys[app.key_to_configure as usize].name));
    }

    draw_background();
}

const PALETTE_ENTRY: &str = "Palette";
const SYNC_ENTRY: &str = "SyncGambatte";

pub fn load_configuration() -> std::io::Result<()> {
    let app = unsafe { &mut *APP_POINTER };

    let subkey = app.platform.reg_create_subkey(HKEY_CURRENT_USER, "SOFTWARE\\inputdisplay", KEY_QUERY_VALUE)?;
    for key in app.keys.iter_mut() {
        key.primary_ipt = app.platform.reg_read_u32(subkey, &format!("{}{}", key.reg_entry, "1"))?;
        key.secondary_ipt = app.platform.reg_read_u32(subkey, &format!("{}{}", key.reg_entry, "2"))?;
    }

    change_palette(app.platform.reg_read_u32(subkey, PALETTE_ENTRY)? as usize);
    if app.platform.reg_read_u32(subkey, SYNC_ENTRY)? > 0 {
        sync_gambatte_keybindings().ok();
    }

    app.platform.reg_close_subkey(subkey)?;

    Ok(())
}

pub fn save_configuration() -> std::io::Result<()> {
    let app = unsafe { &mut *APP_POINTER };

    let subkey = app.platform.reg_create_subkey(HKEY_CURRENT_USER, "SOFTWARE\\inputdisplay", KEY_SET_VALUE)?;
    for key in app.keys.iter() {
        app.platform.reg_write_u32(subkey, &format!("{}{}", key.reg_entry, "1"), key.primary_ipt)?;
        app.platform.reg_write_u32(subkey, &format!("{}{}", key.reg_entry, "2"), key.secondary_ipt)?;
    }
    app.platform.reg_write_u32(subkey, PALETTE_ENTRY, app.palette_index as u32)?;
    app.platform.reg_write_u32(subkey, SYNC_ENTRY, app.gambatte_sync as u32)?;
    app.platform.reg_close_subkey(subkey)?;

    return Ok(());
}
