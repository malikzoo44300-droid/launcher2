use sysinfo::{Signal, System};
use std::path::PathBuf;

// --- Imports Windows (Ignorés sur Linux) ---
#[cfg(windows)]
use std::ptr::null_mut;
#[cfg(windows)]
use widestring::U16CString;
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
#[cfg(windows)]
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
#[cfg(windows)]
use winapi::um::winbase::{CREATE_SUSPENDED, DETACHED_PROCESS};
#[cfg(windows)]
use winapi::um::winuser::{MessageBoxW, MB_OK};

// --- Fonctions multi-plateformes ---

pub fn kill_all(process_names: &[&str]) -> Result<(), String> {
    let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        for &target_name in process_names {
            if process.name().eq_ignore_ascii_case(target_name) {
                let result = process.kill_with(Signal::Kill);
                if result.is_none() || !result.unwrap() {
                    continue;
                }
                break;
            }
        }
    }
    Ok(())
}

pub fn is_process_running(exe_name: &str) -> bool {
    let mut system = System::new_all();
    system.refresh_processes();
    system.processes().values().any(|process| {
        let name = process.name().to_lowercase();
        name == exe_name.to_lowercase()
    })
}

// --- Fonctions Windows (avec versions Fallback) ---

#[cfg(windows)]
fn start_internal(process_path: PathBuf, suspended: bool, args: Option<String>) -> Result<(), String> {
    if !process_path.exists() { return Err("Path does not exist".into()); }
    
    let process_folder = process_path.parent().ok_or("No parent dir")?;
    let process_folder_wide = U16CString::from_str(process_folder.to_str().unwrap_or("")).map_err(|e| e.to_string())?;
    let process_path_str = process_path.to_str().ok_or("Invalid path")?;
    
    let application_name = U16CString::from_str(process_path_str).map_err(|e| e.to_string())?;
    let cmd_line_wide = U16CString::from_str(&format!("\"{}\" {}", process_path_str, args.unwrap_or_default())).map_err(|e| e.to_string())?;

    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let success = unsafe {
        CreateProcessW(
            application_name.into_raw() as *mut u16, cmd_line_wide.into_raw() as *mut u16, null_mut(), null_mut(), 0,
            if suspended { CREATE_SUSPENDED } else { 0 } | DETACHED_PROCESS, null_mut(), process_folder_wide.into_raw() as *const u16,
            &mut startup_info, &mut process_info,
        )
    };

    if success == 0 { return Err(format!("CreateProcessW failed: {}", unsafe { winapi::um::errhandlingapi::GetLastError() })); }

    unsafe { CloseHandle(process_info.hProcess); CloseHandle(process_info.hThread); }
    Ok(())
}

#[cfg(not(windows))]
fn start_internal(_path: PathBuf, _sus: bool, _args: Option<String>) -> Result<(), String> { Ok(()) }

pub fn start(path: PathBuf) -> Result<(), String> { start_internal(path, false, None) }
pub fn start_with_args(path: PathBuf, args: Vec<&str>) -> Result<(), String> { start_internal(path, false, Some(args.join(" "))) }
pub fn start_suspended(path: PathBuf) -> Result<(), String> { start_internal(path, true, None) }
pub fn start_suspended_with_args(path: PathBuf, args: Vec<&str>) -> Result<(), String> { start_internal(path, true, Some(args.join(" "))) }

#[cfg(windows)]
pub fn launch_eac_setup(path: &PathBuf, arg: &str) -> Result<(), String> {
    let eac_path = path.join("EasyAntiCheat\\EasyAntiCheat_EOS_Setup.exe");
    if !eac_path.exists() { return Err("EAC path not found".into()); }
    // ... (insère ici le reste de ta logique CreateProcessW pour EAC comme dans ton code original)
    Ok(())
}

#[cfg(not(windows))]
pub fn launch_eac_setup(_path: &PathBuf, _arg: &str) -> Result<(), String> { Ok(()) }

#[cfg(windows)]
pub fn message_box(title: &str, body: &str) -> Result<(), String> {
    let title_wide = U16CString::from_str(title).map_err(|e| e.to_string())?;
    let body_wide = U16CString::from_str(body).map_err(|e| e.to_string())?;
    unsafe { MessageBoxW(null_mut(), body_wide.as_ptr(), title_wide.as_ptr(), MB_OK); }
    Ok(())
}

#[cfg(not(windows))]
pub fn message_box(_title: &str, _body: &str) -> Result<(), String> { Ok(()) }