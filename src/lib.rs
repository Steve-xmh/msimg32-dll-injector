use path_absolutize::*;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{BOOL, HMODULE},
        System::{LibraryLoader::LoadLibraryW, SystemServices::DLL_PROCESS_ATTACH},
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
};

mod proxy_functions;

#[no_mangle]
extern "system" fn DllMain(
    dll_module: HMODULE,
    reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            if let Err(err) = unsafe { proxy_functions::init_proxy_functions(dll_module) } {
                unsafe {
                    MessageBoxW(
                        None,
                        &HSTRING::from(err.to_string()),
                        &HSTRING::from("msimg32 劫持注入失败"),
                        MB_OK,
                    );
                }
            }
            let inject_dll_name = std::fs::read_to_string("inject.txt").unwrap_or_default();

            for inject_dll_name in inject_dll_name.lines() {
                let inject_dll_path = std::path::Path::new(inject_dll_name);
                if inject_dll_path.is_file() {
                    if let Ok(inject_dll_path) = inject_dll_path.absolutize() {
                        unsafe {
                            let inject_dll_path = inject_dll_path.to_string_lossy().to_string();
                            let _ = LoadLibraryW(&HSTRING::from(inject_dll_path));
                        }
                    }
                }
            }

            true.into()
        }
        _ => true.into(),
    }
}
