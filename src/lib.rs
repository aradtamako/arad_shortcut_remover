use std::fs;

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls};
// use windows::Win32::System::Console::AllocConsole;

pub static mut MODULE: HMODULE = HMODULE(std::ptr::null_mut());

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(hinstDLL: HINSTANCE , reason: u32, _: *mut ()) -> bool
{
  unsafe {
    MODULE = HMODULE(hinstDLL.0 as *mut std::ffi::c_void);
    let _ = DisableThreadLibraryCalls(MODULE);
    // let _ = AllocConsole();
  }

  match reason {
    windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH => {
      let _ = std::thread::spawn(|| {
        on_dll_process_attach()
      });
    },
    _ => ()
  }

  true
}

fn on_dll_process_attach() {
  if let Some(mut path) = dirs::desktop_dir() {
    path.push("Arad.lnk");
    // println!("{:?}", path.display());

    let _ = fs::remove_file(path);
  }
}
