use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub file_type: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub extension: String,
}

fn get_file_type(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "ico" => "image".to_string(),
        "pdf" => "pdf".to_string(),
        "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "ts" | "rs" | "py" | "java"
        | "c" | "cpp" | "h" | "hpp" | "go" | "rb" | "php" | "sh" | "yaml" | "yml" | "toml"
        | "ini" | "cfg" | "conf" | "log" | "csv" => "text".to_string(),
        _ => "other".to_string(),
    }
}

/// Select a folder using native file dialog (cross-platform)
/// Uses spawn_blocking to avoid freezing the main thread
#[tauri::command]
async fn select_folder() -> Option<String> {
    tauri::async_runtime::spawn_blocking(|| {
        FileDialogBuilder::new()
            .set_title("Select a folder to clean up")
            .pick_folder()
            .map(|path| path.to_string_lossy().to_string())
    })
    .await
    .ok()
    .flatten()
}

/// Reveal a file or folder in the system's file explorer (cross-platform)
#[tauri::command]
fn reveal_in_explorer(path: String) -> Result<(), String> {
    opener::reveal(&path).map_err(|e| format!("Failed to reveal in explorer: {}", e))
}

#[tauri::command]
fn list_files(folder_path: String) -> Result<Vec<FileInfo>, String> {
    let path = Path::new(&folder_path);
    
    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    
    let mut files: Vec<FileInfo> = Vec::new();
    
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                
                // Skip directories and hidden files
                if entry_path.is_dir() {
                    continue;
                }
                
                let file_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip hidden files (starting with .)
                if file_name.starts_with('.') {
                    continue;
                }
                
                let extension = entry_path
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                let file_type = get_file_type(&extension);
                
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                
                files.push(FileInfo {
                    path: entry_path.to_string_lossy().to_string(),
                    name: file_name,
                    file_type,
                    size,
                });
            }
        }
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    }
    
    // Sort files by name
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    
    Ok(files)
}

#[tauri::command]
fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    
    let file_type = get_file_type(&extension);
    
    let size = fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get metadata: {}", e))?;
    
    Ok(FileMetadata {
        path: file_path,
        name,
        file_type,
        size,
        extension,
    })
}

#[tauri::command]
fn rename_file(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|e| format!("Failed to rename file: {}", e))
}

#[tauri::command]
fn undo_rename(current_path: String, original_path: String) -> Result<(), String> {
    fs::rename(&current_path, &original_path).map_err(|e| format!("Failed to undo rename: {}", e))
}

#[tauri::command]
fn read_text_preview(file_path: String, max_bytes: usize) -> Result<String, String> {
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    
    let preview: String = content.chars().take(max_bytes).collect();
    
    Ok(preview)
}

#[derive(Debug, Serialize)]
pub struct DeleteResult {
    pub success: Vec<String>,
    pub failed: Vec<(String, String)>, // (path, error message)
}

#[tauri::command]
fn delete_files(file_paths: Vec<String>) -> DeleteResult {
    let mut success = Vec::new();
    let mut failed = Vec::new();
    
    for file_path in file_paths {
        let path = Path::new(&file_path);
        
        if !path.exists() {
            failed.push((file_path, "File does not exist".to_string()));
            continue;
        }
        
        match fs::remove_file(path) {
            Ok(_) => success.push(file_path),
            Err(e) => failed.push((file_path, e.to_string())),
        }
    }
    
    DeleteResult { success, failed }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            select_folder,
            list_files,
            get_file_metadata,
            rename_file,
            undo_rename,
            read_text_preview,
            delete_files,
            reveal_in_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

