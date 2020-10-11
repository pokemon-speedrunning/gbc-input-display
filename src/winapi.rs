#![allow(dead_code)]

use std::ffi::OsStr;
use std::os::windows::prelude::*;

pub type WindowProc = unsafe extern "system" fn(usize, u32, usize, usize) -> u32;
pub type HookCallback = unsafe extern "system" fn(i32, usize, usize) -> usize;

#[repr(C)]
pub struct WindowClassW {
    pub style: u32,
    pub window_proc: WindowProc,
    pub cls_extra: u32,
    pub wnd_extra: u32,
    pub instance: usize,
    pub icon: usize,
    pub cursor: usize,
    pub brush: usize,
    pub menu_name: *const u16,
    pub class_name: *const u16,
}

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct Message {
    pub window: usize,
    pub message: u32,
    pub wparam: usize,
    pub lparam: usize,
    pub time: u32,
    pub point: Point,
    pub private: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct BitmapInfo {
    pub header: BitmapInfoHeader,
    pub colors: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub colors_used: u32,
    pub colors_important: u32,
}

pub fn to_unicode(s: &str) -> Vec<u16> {
    return OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect();
}

#[link(name = "user32")]
extern "C" {
    pub fn RegisterClassW(window_class: &WindowClassW) -> u16;
    pub fn AdjustWindowRect(rect: &mut Rect, style: u32, menu: bool);
    pub fn CreateWindowExW(
        extended_style: u32,
        class_name: *const u16,
        window_name: *const u16,
        style: u32,
        x: u32,
        y: u32,
        width: i32,
        height: i32,
        window_parent: usize,
        menu: usize,
        instance: usize,
        void: *const u8,
    ) -> usize;

    pub fn DefWindowProcW(window: usize, message: u32, wparam: usize, lparam: usize) -> u32;
    pub fn GetMessageW(message: &mut Message, window: usize, message_filter_min: u32, message_filter_max: u32) -> bool;
    pub fn TranslateMessage(message: &mut Message);
    pub fn DispatchMessageW(message: &mut Message);
    pub fn PostQuitMessage(exit_code: i32);

    pub fn SetWindowsHookExW(hook_type: u32, callback: HookCallback, instance: usize, thread_id: u32) -> usize;
    pub fn CallNextHookEx(hook: usize, code: i32, wparam: usize, lparam: usize) -> usize;

    pub fn GetDC(window: usize) -> usize;
    pub fn ReleaseDC(window: usize, device_context: usize);

    pub fn CreatePopupMenu() -> usize;
    pub fn AppendMenuW(menu: usize, flags: u32, id: u32, name: *const u16);
    pub fn TrackPopupMenu(menu: usize, flags: u32, x: i32, y: i32, reserved: i32, window: usize, reserved: *const Rect) -> u32;

    pub fn GetCursorPos(point: &mut Point);
    pub fn GetKeyState(key_code: u32) -> u16;
}

#[link(name = "gdi32")]
extern "C" {
    pub fn StretchDIBits(
        device_context: usize,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_heigh: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        bits: *const u8,
        bitmap_info: &BitmapInfo,
        usage: u32,
        rop: u32,
    );
}

#[link(name = "kernel32")]
extern "C" {
    pub fn GetModuleHandleW(module_name: *const u16) -> usize;
    pub fn GetLastError() -> u32;
}

#[link(name = "advapi32")]
extern "C" {
    pub fn RegOpenKeyExW(hkey: usize, sub_key: *const u16, options: u32, access: u32, result: &mut usize) -> i32;
    pub fn RegCreateKeyExA(hkey: usize, sub_key: *const u16, reserved: u32, class: *const u8, options: u32, access: u32, security_attributes: *const u8, result: &mut usize, disposition: &mut u32) -> i32;
    pub fn RegCloseKey(hkey: usize) -> i32;
    pub fn RegQueryValueExW(hkey: usize, sub_key: *const u16, reserved: *const u8, data_type: &mut u32, data: *mut u8, data_size: &mut u32) -> i32;
    pub fn RegSetValueExW(hkey: usize, sub_key: *const u16, reserved: u32, data_type: u32, data: *const u32, data_size: u32) -> i32;
}

pub const CS_VREDRAW: u32 = 0x0001;
pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_DBLCLKS: u32 = 0x0008;
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_CLASSDC: u32 = 0x0040;
pub const CS_PARENTDC: u32 = 0x0080;
pub const CS_NOCLOSE: u32 = 0x0200;
pub const CS_SAVEBITS: u32 = 0x0800;
pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;
pub const CS_GLOBALCLASS: u32 = 0x4000;
pub const CW_USEDEFAULT: u32 = 0x80000000;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_MINIMIZE: u32 = 0x20000000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_DISABLED: u32 = 0x08000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_MAXIMIZE: u32 = 0x01000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_BORDER: u32 = 0x00800000;
pub const WS_DLGFRAME: u32 = 0x00400000;
pub const WS_VSCROLL: u32 = 0x00200000;
pub const WS_HSCROLL: u32 = 0x00100000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_GROUP: u32 = 0x00020000;
pub const WS_TABSTOP: u32 = 0x00010000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;

pub const WM_NULL: u32 = 0x0000;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_MOVE: u32 = 0x0003;
pub const WM_SIZE: u32 = 0x0005;
pub const WA_INACTIVE: u32 = 0;
pub const WA_ACTIVE: u32 = 1;
pub const WA_CLICKACTIVE: u32 = 2;
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_ENABLE: u32 = 0x000A;
pub const WM_SETREDRAW: u32 = 0x000B;
pub const WM_SETTEXT: u32 = 0x000C;
pub const WM_GETTEXT: u32 = 0x000D;
pub const WM_GETTEXTLENGTH: u32 = 0x000E;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ERASEBKGND: u32 = 0x0014;
pub const WM_SYSCOLORCHANGE: u32 = 0x0015;
pub const WM_SHOWWINDOW: u32 = 0x0018;
pub const WM_WININICHANGE: u32 = 0x001A;
pub const WM_DEVMODECHANGE: u32 = 0x001B;
pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_FONTCHANGE: u32 = 0x001D;
pub const WM_TIMECHANGE: u32 = 0x001E;
pub const WM_CANCELMODE: u32 = 0x001F;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_MOUSEACTIVATE: u32 = 0x0021;
pub const WM_CHILDACTIVATE: u32 = 0x0022;
pub const WM_QUEUESYNC: u32 = 0x0023;
pub const WM_GETMINMAXINFO: u32 = 0x0024;
pub const WM_PAINTICON: u32 = 0x0026;
pub const WM_ICONERASEBKGND: u32 = 0x0027;
pub const WM_NEXTDLGCTL: u32 = 0x0028;
pub const WM_SPOOLERSTATUS: u32 = 0x002A;
pub const WM_DRAWITEM: u32 = 0x002B;
pub const WM_MEASUREITEM: u32 = 0x002C;
pub const WM_DELETEITEM: u32 = 0x002D;
pub const WM_VKEYTOITEM: u32 = 0x002E;
pub const WM_CHARTOITEM: u32 = 0x002F;
pub const WM_SETFONT: u32 = 0x0030;
pub const WM_GETFONT: u32 = 0x0031;
pub const WM_SETHOTKEY: u32 = 0x0032;
pub const WM_GETHOTKEY: u32 = 0x0033;
pub const WM_QUERYDRAGICON: u32 = 0x0037;
pub const WM_COMPAREITEM: u32 = 0x0039;
pub const WM_COMPACTING: u32 = 0x0041;
pub const WM_WINDOWPOSCHANGING: u32 = 0x0046;
pub const WM_WINDOWPOSCHANGED: u32 = 0x0047;
pub const WM_COPYDATA: u32 = 0x004A;
pub const WM_CANCELJOURNAL: u32 = 0x004B;
pub const WM_KEYFIRST: u32 = 0x0100;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_DEADCHAR: u32 = 0x0103;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_SYSCHAR: u32 = 0x0106;
pub const WM_SYSDEADCHAR: u32 = 0x0107;
pub const WM_MOUSEFIRST: u32 = 0x0200;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_RBUTTONDBLCLK: u32 = 0x0206;
pub const WM_MBUTTONDOWN: u32 = 0x0207;
pub const WM_MBUTTONUP: u32 = 0x0208;
pub const WM_MBUTTONDBLCLK: u32 = 0x0209;

pub const WH_JOURNALRECORD: u32 = 0;
pub const WH_JOURNALPLAYBACK: u32 = 1;
pub const WH_KEYBOARD: u32 = 2;
pub const WH_GETMESSAGE: u32 = 3;
pub const WH_CALLWNDPROC: u32 = 4;
pub const WH_CBT: u32 = 5;
pub const WH_SYSMSGFILTER: u32 = 6;
pub const WH_MOUSE: u32 = 7;
pub const WH_DEBUG: u32 = 9;
pub const WH_SHELL: u32 = 10;
pub const WH_FOREGROUNDIDLE: u32 = 11;
pub const WH_KEYBOARD_LL: u32 = 13;
pub const WH_MOUSE_LL: u32 = 14;

pub const BI_RGB: u32 = 0;
pub const BI_RLE8: u32 = 1;
pub const BI_RLE4: u32 = 2;
pub const BI_BITFIELDS: u32 = 3;
pub const BI_JPEG: u32 = 4;
pub const BI_PNG: u32 = 5;

pub const DIB_RGB_COLORS: u32 = 0;
pub const DIB_PAL_COLORS: u32 = 1;

pub const SRCCOPY: u32 = 0x00CC0020;
pub const SRCPAINT: u32 = 0x00EE0086;
pub const SRCAND: u32 = 0x008800C6;
pub const SRCINVERT: u32 = 0x00660046;
pub const SRCERASE: u32 = 0x00440328;
pub const NOTSRCCOPY: u32 = 0x00330008;
pub const NOTSRCERASE: u32 = 0x001100A6;
pub const MERGECOPY: u32 = 0x00C000CA;
pub const MERGEPAINT: u32 = 0x00BB0226;
pub const PATCOPY: u32 = 0x00F00021;
pub const PATPAINT: u32 = 0x00FB0A09;
pub const PATINVERT: u32 = 0x005A0049;
pub const DSTINVERT: u32 = 0x00550009;
pub const BLACKNESS: u32 = 0x00000042;
pub const WHITENESS: u32 = 0x00FF0062;

pub const MF_INSERT: u32 = 0x00000000;
pub const MF_CHANGE: u32 = 0x00000080;
pub const MF_APPEND: u32 = 0x00000100;
pub const MF_DELETE: u32 = 0x00000200;
pub const MF_REMOVE: u32 = 0x00001000;
pub const MF_BYCOMMAND: u32 = 0x00000000;
pub const MF_BYPOSITION: u32 = 0x00000400;
pub const MF_SEPARATOR: u32 = 0x00000800;
pub const MF_ENABLED: u32 = 0x00000000;
pub const MF_GRAYED: u32 = 0x00000001;
pub const MF_DISABLED: u32 = 0x00000002;
pub const MF_UNCHECKED: u32 = 0x00000000;
pub const MF_CHECKED: u32 = 0x00000008;
pub const MF_USECHECKBITMAPS: u32 = 0x00000200;
pub const MF_STRING: u32 = 0x00000000;
pub const MF_BITMAP: u32 = 0x00000004;
pub const MF_OWNERDRAW: u32 = 0x00000100;
pub const MF_POPUP: u32 = 0x00000010;
pub const MF_MENUBARBREAK: u32 = 0x00000020;
pub const MF_MENUBREAK: u32 = 0x00000040;
pub const MF_UNHILITE: u32 = 0x00000000;
pub const MF_HILITE: u32 = 0x00000080;

pub const TPM_LEFTBUTTON: u32 = 0x0000;
pub const TPM_RIGHTBUTTON: u32 = 0x0002;
pub const TPM_LEFTALIGN: u32 = 0x0000;
pub const TPM_CENTERALIGN: u32 = 0x0004;
pub const TPM_RIGHTALIGN: u32 = 0x0008;
pub const TPM_TOPALIGN: u32 = 0x0000;
pub const TPM_VCENTERALIGN: u32 = 0x0010;
pub const TPM_BOTTOMALIGN: u32 = 0x0020;
pub const TPM_HORIZONTAL: u32 = 0x0000;
pub const TPM_VERTICAL: u32 = 0x0040;
pub const TPM_NONOTIFY: u32 = 0x0080;
pub const TPM_RETURNCMD: u32 = 0x0100;
pub const TPM_RECURSE: u32 = 0x0001;
pub const TPM_HORPOSANIMATION: u32 = 0x0400;
pub const TPM_HORNEGANIMATION: u32 = 0x0800;
pub const TPM_VERPOSANIMATION: u32 = 0x1000;
pub const TPM_VERNEGANIMATION: u32 = 0x2000;
pub const TPM_NOANIMATION: u32 = 0x4000;
pub const TPM_LAYOUTRTL: u32 = 0x8000;
pub const TPM_WORKAREA: u32 = 0x10000;

pub const VK_CONTROL: u32 = 162;

pub const HKEY_CLASSES_ROOT: usize = 0x80000000;
pub const HKEY_CURRENT_USER: usize = 0x80000001;
pub const HKEY_LOCAL_MACHINE: usize = 0x80000002;
pub const HKEY_USERS: usize = 0x80000003;
pub const HKEY_PERFORMANCE_DATA: usize = 0x80000004;
pub const HKEY_PERFORMANCE_TEXT: usize = 0x80000050;
pub const HKEY_PERFORMANCE_NLSTEXT: usize = 0x80000060;
pub const HKEY_CURRENT_CONFIG: usize = 0x80000005;
pub const HKEY_DYN_DATA: usize = 0x80000006;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: usize = 0x80000007;

pub const KEY_QUERY_VALUE: u32 = 0x00000001;
pub const KEY_SET_VALUE: u32 = 0x00000002;
pub const KEY_CREATE_SUB_KEY: u32 = 0x00000004;
pub const KEY_ENUMERATE_SUB_KEYS: u32 = 0x00000008;
pub const KEY_NOTIFY: u32 = 0x00000010;
pub const KEY_CREATE_LINK: u32 = 0x00000020;
pub const KEY_WOW64_64KEY: u32 = 0x00000100;
pub const KEY_WOW64_32KEY: u32 = 0x00000200;
pub const KEY_WOW64_RES: u32 = 0x00000300;
pub const KEY_ALL_ACCESS: u32 = 0xF003F;

pub const REG_OPTION_NON_VOLATILE: u32 = 0x00000000;
pub const REG_OPTION_VOLATILE: u32 = 0x00000001;
pub const REG_OPTION_CREATE_LINK: u32 = 0x00000002;
pub const REG_OPTION_BACKUP_RESTORE: u32 = 0x00000004;
pub const REG_OPTION_OPEN_LINK: u32 = 0x00000008;
pub const REG_NONE: u32 = 0;
pub const REG_SZ: u32 = 1;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_BINARY: u32 = 3;
pub const REG_DWORD: u32 = 4;
pub const REG_DWORD_LITTLE_ENDIAN: u32 = 4;
pub const REG_DWORD_BIG_ENDIAN: u32 = 5;
pub const REG_LINK: u32 = 6;
pub const REG_MULTI_SZ: u32 = 7;
pub const REG_RESOURCE_LIST: u32 = 8;
pub const REG_FULL_RESOURCE_DESCRIPTOR: u32 = 9;
pub const REG_RESOURCE_REQUIREMENTS_LIST: u32 = 10;
pub const REG_QWORD: u32 = 11;
pub const REG_QWORD_LITTLE_ENDIAN: u32 = 11;
