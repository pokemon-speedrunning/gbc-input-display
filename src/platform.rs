use crate::gfx::*;
use crate::winapi::*;

use std::collections::*;
use std::mem::*;
use std::ptr::*;

pub type MessageCallback = fn(usize, usize);

pub struct Platform {
    pub instance: usize,
    pub running: bool,
    pub window_width: i32,
    pub window_height: i32,
    pub window_handle: usize,
    pub windows_message_callbacks: HashMap<u32, MessageCallback>,
    pub windows_hooks: Vec<usize>,
    pub offscreen_buffer: BackBuffer,
}

#[derive(Debug)]
pub enum PlatfromError {
    WindowClassCreation,
    WindowHandleCreation,
}

pub enum MenuItem {
    Unchecked(String),
    Checked(String),
    Seperator,
    SubMenu(String, usize),
}

#[derive(PartialEq)]
pub enum KeyState {
    None,
    Pressed,
    Released,
}

macro_rules! werr {
    ($e:expr) => {
        Err(std::io::Error::from_raw_os_error($e as i32))
    };
}

impl Platform {
    pub fn new(width: i32, height: i32, scale: i32, title: &str) -> Result<Platform, PlatfromError> {
        unsafe {
            let instance = GetModuleHandleW(null());
            let win_width = width * scale;
            let win_height = height * scale;
            let win_handle = Platform::create_window(instance, win_width, win_height, title)?;

            return Ok(Platform {
                instance: instance,
                running: true,
                window_width: win_width,
                window_height: win_height,
                window_handle: win_handle,
                windows_message_callbacks: HashMap::new(),
                windows_hooks: Vec::new(),
                offscreen_buffer: BackBuffer::new(width, height),
            });
        }
    }

    pub fn register_callback(&mut self, message_type: u32, callback: MessageCallback) {
        self.windows_message_callbacks.insert(message_type, callback);
    }

    pub fn register_hook(&mut self, hook_type: u32, callback: HookCallback) {
        unsafe { self.windows_hooks.push(SetWindowsHookExW(hook_type, callback, self.instance, 0)) };
    }

    pub fn start_message_queue(&mut self) {
        unsafe {
            let mut message: Message = zeroed();

            while GetMessageW(&mut message, 0, 0, 0) {
                TranslateMessage(&mut message);
                DispatchMessageW(&mut message);

                if let Some(callback) = self.windows_message_callbacks.get(&message.message) {
                    callback(message.wparam, message.lparam);
                }
            }
        }
    }

    pub fn update_window(&mut self) {
        unsafe {
            let dc = GetDC(self.window_handle);
            StretchDIBits(
                dc,
                0,
                0,
                self.window_width,
                self.window_height,
                0,
                0,
                self.offscreen_buffer.width,
                self.offscreen_buffer.height,
                self.offscreen_buffer.memory.as_ptr(),
                &self.offscreen_buffer.info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            ReleaseDC(self.window_handle, dc);
        }
    }

    pub fn show_menu(&self, items: &[MenuItem], item_counter: &mut u32) -> u32 {
        unsafe {
            let menu = self.create_menu(items, item_counter);
            let mut cursor_pos: Point = zeroed();
            GetCursorPos(&mut cursor_pos);
            return TrackPopupMenu(menu, TPM_RETURNCMD, cursor_pos.x, cursor_pos.y, 0, self.window_handle, null());
        }
    }

    pub fn create_menu(&self, items: &[MenuItem], item_counter: &mut u32) -> usize {
        unsafe {
            let menu = CreatePopupMenu();

            for menu_item in items.iter() {
                *item_counter += 1;
                match menu_item {
                    MenuItem::Unchecked(name) => AppendMenuW(menu, MF_STRING, *item_counter, to_unicode(name).as_ptr()),
                    MenuItem::Checked(name) => AppendMenuW(menu, MF_CHECKED, *item_counter, to_unicode(name).as_ptr()),
                    MenuItem::Seperator => AppendMenuW(menu, MF_SEPARATOR, *item_counter, null()),
                    MenuItem::SubMenu(name, submenu) => AppendMenuW(menu, MF_STRING | MF_POPUP, *submenu as u32, to_unicode(name).as_ptr()),
                }
            }

            return menu;
        }
    }

    pub fn is_key_down(&self, code: u32) -> bool {
        unsafe { return (GetKeyState(code) & 0x8000) > 0 };
    }

    pub fn key_state_from_wparam(&self, wparam: usize) -> KeyState {
        match wparam as u32 {
            WM_KEYDOWN => KeyState::Pressed,
            WM_SYSKEYDOWN => KeyState::Pressed,
            WM_KEYUP => KeyState::Released,
            WM_SYSKEYUP => KeyState::Released,
            _ => KeyState::None,
        }
    }

    pub fn reg_create_subkey(&self, hkey: usize, sub_key: &str, access: u32) -> std::io::Result<usize> {
        unsafe {
            let mut new_hkey: usize = 0;
            let mut disposition: u32 = 0;
            match RegCreateKeyExA(hkey, to_unicode(sub_key).as_ptr(), 0, null(), REG_OPTION_NON_VOLATILE, access, null(), &mut new_hkey, &mut disposition) {
                0 => Ok(new_hkey),
                err => werr!(err),
            }
        }
    }

    pub fn reg_open_subkey(&self, hkey: usize, sub_key: &str, access: u32) -> std::io::Result<usize> {
        unsafe {
            let mut new_hkey: usize = 0;
            match RegOpenKeyExW(hkey, to_unicode(sub_key).as_ptr(), 0, access, &mut new_hkey) {
                0 => Ok(new_hkey),
                err => werr!(err),
            }
        }
    }

    pub fn reg_close_subkey(&self, hkey: usize) -> std::io::Result<()> {
        unsafe {
            match RegCloseKey(hkey) {
                0 => Ok(()),
                err => werr!(err),
            }
        }
    }

    pub fn reg_read_u32(&self, hkey: usize, sub_key: &str) -> std::io::Result<u32> {
        unsafe {
            let mut buf_len = size_of::<u32>() as u32;
            let mut buf_type = 0;
            let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
            match RegQueryValueExW(hkey, to_unicode(sub_key).as_ptr(), null(), &mut buf_type, buf.as_mut_ptr(), &mut buf_len) {
                0 => Ok(*(buf.as_ptr() as *const u32)),
                err => werr!(err),
            }
        }
    }

    pub fn reg_write_u32(&self, hkey: usize, sub_key: &str, value: u32) -> std::io::Result<()> {
        unsafe {
            match RegSetValueExW(hkey, to_unicode(sub_key).as_ptr(), 0, REG_DWORD, &value, size_of::<u32>() as u32) {
                0 => Ok(()),
                err => werr!(err),
            }
        }
    }

    unsafe fn create_window(instance: usize, width: i32, height: i32, title: &str) -> Result<usize, PlatfromError> {
        let class = to_unicode(&format!("{}{}", title, "Class"));
        let title = to_unicode(title);

        let mut window_class = WindowClassW {
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            window_proc: Platform::windows_message_callback,
            cls_extra: 0,
            wnd_extra: 0,
            instance: instance,
            icon: 0,
            cursor: 0,
            brush: 0,
            menu_name: null(),
            class_name: class.as_ptr(),
        };

        if RegisterClassW(&mut window_class) == 0 {
            return Err(PlatfromError::WindowClassCreation);
        }

        let window_style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE;
        let mut window_dimension = Rect { left: 0, top: 0, right: width, bottom: height };
        AdjustWindowRect(&mut window_dimension, window_style, false);

        match CreateWindowExW(
            0,
            class.as_ptr(),
            title.as_ptr(),
            window_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_dimension.right - window_dimension.left,
            window_dimension.bottom - window_dimension.top,
            0,
            0,
            instance,
            null(),
        ) {
            0 => return Err(PlatfromError::WindowHandleCreation),
            x => return Ok(x),
        };
    }

    unsafe extern "system" fn windows_message_callback(window: usize, message: u32, wparam: usize, lparam: usize) -> u32 {
        match message {
            WM_CLOSE => PostQuitMessage(0),
            _ => return DefWindowProcW(window, message, wparam, lparam),
        }

        return 0;
    }
}
