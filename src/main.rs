extern crate winapi;

use std::env;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::um::winuser::{
  CallNextHookEx, DispatchMessageW, GetMessageW, PostQuitMessage, SetWindowsHookExW,
  TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN,
};

static mut DISABLE_LEFT: bool = true;
static mut DISABLE_RIGHT: bool = true;

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut disable_left = false;
  let mut disable_right = false;
  
  if args.contains(&"--disable-left".to_string()) {
    disable_left = true;
  }
  
  if args.contains(&"--disable-right".to_string()) {
    disable_right = true;
  }
  
  if !disable_left && !disable_right {
    disable_left = true;
    disable_right = true;
  }

  println!(
    "To disable specific Windows keys, use the flags:\n\
    --disable-left: Disables only the left Windows key\n\
    --disable-right: Disables only the right Windows key\n\
    If no flags are provided, both Windows keys will be disabled by default.\n\
    github.com/nous-/disable-windows-key
    -------"
  );  

  println!(
    "{}",
    if disable_left && disable_right {
      "Left and right Windows keys disabled"
    } else if disable_left {
      "Left Windows key disabled"
    } else {
      "Right Windows key disabled"
    }
  );

  println!("Ctrl+C to exit and re-enable.");

  unsafe {
    DISABLE_LEFT = disable_left;
    DISABLE_RIGHT = disable_right;
  }

  let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), null_mut(), 0) };

  if hook.is_null() {
    panic!("Failed to set keyboard hook");
  }

  loop {
    let mut msg = winapi::um::winuser::MSG {
      hwnd: null_mut(),
      message: 0 as UINT,
      wParam: 0 as WPARAM,
      lParam: 0 as LPARAM,
      time: 0 as DWORD,
      pt: winapi::shared::windef::POINT { x: 0, y: 0 },
    };

    let get_result = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };

    if get_result == -1 {
      panic!("Failed to get message");
    }

    unsafe {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }

    if msg.message == winapi::um::winuser::WM_QUIT {
      break;
    }

    thread::sleep(Duration::from_millis(10));
  }

  unsafe {
    UnhookWindowsHookEx(hook);
    PostQuitMessage(0);
  }
}

unsafe extern "system" fn keyboard_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  if code >= 0 {
    let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
    let vk_code = kb_struct.vkCode as i32;

    if (DISABLE_LEFT && vk_code == winapi::um::winuser::VK_LWIN as i32)
      || (DISABLE_RIGHT && vk_code == winapi::um::winuser::VK_RWIN as i32)
    {
      if w_param == WM_KEYDOWN as WPARAM {
        return 1; // Block the keypress
      }
    }
  }
  CallNextHookEx(null_mut(), code, w_param, l_param)
}
