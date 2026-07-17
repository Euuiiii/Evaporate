use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn secure_wipe(path: &PathBuf) -> Result<(), std::io::Error> {
    if path.exists() && path.is_file() {
        if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
            if let Ok(metadata) = fs::metadata(path) {
                let zeros = vec![0u8; metadata.len() as usize];
                let _ = file.write_all(&zeros);
            }
        }
        fs::remove_file(path)?;
    }
    Ok(())
}

fn wipe_matching_files_in_dir(dir: &Path, filename_target: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten(){
            let path = entry.path();

            if path.is_dir() {
                wipe_matching_files_in_dir(&path, filename_target);
            } else if path.file_name().map_or(false, |name |name == filename_target){
                let _ = secure_wipe(&path);
            }
        }
    }
}

fn clean_browser(_win_proc: &str, _unix_proc: &str, base_path: PathBuf, sub_target: &str, is_gecko: bool){
    #[cfg(target_os = "windows")]{
        let _ = Command::new("taskkill").args(&["/F", "/IM", _win_proc]).output();
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]{
        let _ = Command::new("pkill").arg("-f").arg(_unix_proc).output();
    }

    if base_path.exists(){
        if is_gecko {
            wipe_matching_files_in_dir(&base_path, sub_target);
        }else{
            let target_file = base_path.join(sub_target);
            let _ = secure_wipe(&target_file);
        }
    }
}

fn wipe_browser_download_history(base_path: &PathBuf, is_gecko: bool) {
    if base_path.exists() {
        if is_gecko {
            wipe_matching_files_in_dir(base_path, "downloads.sqlite");
        } else {
            let history_file = base_path.join("History");
            let _ = secure_wipe(&history_file);
        }
    }
}

fn wipe_downloads() -> Result<(), std::io::Error> {
    // Windows Downloads 
    #[cfg(target_os = "windows")] {
        if let Ok(username) = env::var("USERNAME") {
            let downloads_path = PathBuf::from("C:\\Users")
                .join(&username)
                .join("Downloads");
            if downloads_path.exists() {
                if let Ok(entries) = fs::read_dir(&downloads_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let _ = if path.is_dir() {
                            fs::remove_dir_all(&path)
                        } else {
                            secure_wipe(&path)
                        };
                    }
                }
            }
        }
    }
    // macOS Downloads 
    #[cfg(target_os = "macos")] {
        if let Ok(home) = env::var("HOME") {
            let downloads_path = PathBuf::from(&home).join("Downloads");
            if downloads_path.exists() {
                if let Ok(entries) = fs::read_dir(&downloads_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let _ = if path.is_dir() {
                            fs::remove_dir_all(&path)
                        } else {
                            secure_wipe(&path)
                        };
                    }
                }
            }
        }
    }
    // Linux Downloads 
    #[cfg(target_os = "linux")] {
        if let Ok(home) = env::var("HOME") {
            let downloads_path = PathBuf::from(&home).join("Downloads");
            if downloads_path.exists() {
                if let Ok(entries) = fs::read_dir(&downloads_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let _ = if path.is_dir() {
                            fs::remove_dir_all(&path)
                        } else {
                            secure_wipe(&path)
                        };
                    }
                }
            }
        }
    }
    Ok(())
}

#[tauri::command] 
fn run_sanitize_routine() -> Result<String, String> {
    // Windows Target Routine execution
    #[cfg(target_os = "windows")] {
        let local_app = env::var("LOCALAPPDATA").unwrap_or_default();
        let app_data = env::var("APPDATA").unwrap_or_default();

        let chrome_base = PathBuf::from(&local_app).join("Google").join("Chrome").join("User Data");
        clean_browser("chrome.exe", "chrome", chrome_base, "Default/Network/Cookies", false);

        let edge_base = PathBuf::from(&local_app).join("Microsoft").join("Edge").join("User Data");
        clean_browser("msedge.exe", "msedge", edge_base, "Default/Network/Cookies", false);

        let ff_base = PathBuf::from(&app_data).join("Mozilla").join("Firefox").join("Profiles");
        clean_browser("firefox.exe", "firefox", ff_base, "cookies.sqlite", true);

        let chrome_history = PathBuf::from(&local_app).join("Google").join("Chrome").join("User Data").join("Default");
        wipe_browser_download_history(&chrome_history, false);
        
        let edge_history = PathBuf::from(&local_app).join("Microsoft").join("Edge").join("User Data").join("Default");
        wipe_browser_download_history(&edge_history, false);
    }
    
    // macOS Target Routine execution
    #[cfg(target_os = "macos")] {
        let home = env::var("HOME").unwrap_or_default();

        let chrome_base = PathBuf::from(&home).join("Library").join("Application Support").join("Google").join("Chrome");
        clean_browser("chrome.exe", "chrome", chrome_base, "Default/Network/Cookies", false);

        let edge_base = PathBuf::from(&home).join("Library").join("Application Support").join("Microsoft Edge");
        clean_browser("msedge.exe", "msedge", edge_base, "Default/Network/Cookies", false);

        let ff_base = PathBuf::from(&home).join("Library").join("Application Support").join("Firefox").join("Profiles");
        clean_browser("firefox.exe", "firefox", ff_base, "cookies.sqlite", true);

        let chrome_history = PathBuf::from(&home).join("Library").join("Application Support").join("Google").join("Chrome").join("Default");
        wipe_browser_download_history(&chrome_history, false);
        
        let edge_history = PathBuf::from(&home).join("Library").join("Application Support").join("Microsoft Edge").join("Default");
        wipe_browser_download_history(&edge_history, false);
        
        let ff_history = PathBuf::from(&home).join("Library").join("Application Support").join("Firefox").join("Profiles");
        wipe_browser_download_history(&ff_history, true);
    }
    
    // Linux Target Routine execution
    #[cfg(target_os = "linux")] {
        let home = env::var("HOME").unwrap_or_default();

        let chrome_base = PathBuf::from(&home).join(".config").join("google-chrome");
        clean_browser("chrome.exe", "chrome", chrome_base, "Default/Network/Cookies", false);

        let edge_base = PathBuf::from(&home).join(".config").join("microsoft-edge");
        clean_browser("msedge.exe", "msedge", edge_base, "Default/Network/Cookies", false);

        let ff_base = PathBuf::from(&home).join(".mozilla").join("firefox");
        clean_browser("firefox.exe", "firefox", ff_base, "cookies.sqlite", true);

        let chrome_history = PathBuf::from(&home).join(".config").join("google-chrome").join("Default");
        wipe_browser_download_history(&chrome_history, false);
        
        let edge_history = PathBuf::from(&home).join(".config").join("microsoft-edge").join("Default");
        wipe_browser_download_history(&edge_history, false);
        
        let ff_history = PathBuf::from(&home).join(".mozilla").join("firefox");
        wipe_browser_download_history(&ff_history, true);
    }

    let _ = wipe_downloads();

    Ok("All selected targets wiped completely.".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_sanitize_routine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}