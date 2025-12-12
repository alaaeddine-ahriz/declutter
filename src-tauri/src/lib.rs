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
    pub is_directory: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct DirectoryNode {
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub children: Option<Vec<DirectoryNode>>,
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

/// Calculate the total size of a directory (recursive)
fn get_directory_size(path: &Path) -> u64 {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                size += get_directory_size(&entry_path);
            } else {
                size += entry.metadata().map(|m| m.len()).unwrap_or(0);
            }
        }
    }
    size
}

/// Build a directory tree structure for preview
fn build_directory_tree(path: &Path, current_depth: usize, max_depth: usize) -> DirectoryNode {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());
    
    let is_directory = path.is_dir();
    
    if !is_directory {
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        return DirectoryNode {
            name,
            is_directory: false,
            size,
            children: None,
        };
    }
    
    let mut total_size = 0;
    let children = if current_depth < max_depth {
        let mut child_nodes: Vec<DirectoryNode> = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let entry_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip hidden files
                if entry_name.starts_with('.') {
                    continue;
                }
                
                let child = build_directory_tree(&entry_path, current_depth + 1, max_depth);
                total_size += child.size;
                child_nodes.push(child);
            }
        }
        // Sort: folders first, then files
        child_nodes.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });
        Some(child_nodes)
    } else {
        // At max depth, just calculate size without children
        total_size = get_directory_size(path);
        None
    };
    
    DirectoryNode {
        name,
        is_directory: true,
        size: total_size,
        children,
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

/// Get a directory tree structure for preview (used for folder previews)
#[tauri::command]
fn list_directory_tree(folder_path: String, max_depth: Option<usize>) -> Result<DirectoryNode, String> {
    let path = Path::new(&folder_path);
    let max_depth = max_depth.unwrap_or(3);
    
    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    
    Ok(build_directory_tree(path, 0, max_depth))
}

/// Get the count of items in a folder (for confirmation dialog)
#[tauri::command]
fn get_folder_contents_count(folder_path: String) -> Result<(usize, usize), String> {
    let path = Path::new(&folder_path);
    
    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }
    
    fn count_recursive(path: &Path) -> (usize, usize) {
        let mut files = 0;
        let mut folders = 0;
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Skip hidden
                if name.starts_with('.') {
                    continue;
                }
                
                if entry_path.is_dir() {
                    folders += 1;
                    let (f, d) = count_recursive(&entry_path);
                    files += f;
                    folders += d;
                } else {
                    files += 1;
                }
            }
        }
        (files, folders)
    }
    
    Ok(count_recursive(path))
}

#[tauri::command]
fn list_files(folder_path: String, include_folders: Option<bool>) -> Result<Vec<FileInfo>, String> {
    let path = Path::new(&folder_path);
    let include_folders = include_folders.unwrap_or(false);
    
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
                let is_dir = entry_path.is_dir();
                
                // Skip directories if not including folders
                if is_dir && !include_folders {
                    continue;
                }
                
                let file_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip hidden files (starting with .)
                if file_name.starts_with('.') {
                    continue;
                }
                
                let (file_type, size) = if is_dir {
                    ("folder".to_string(), get_directory_size(&entry_path))
                } else {
                    let extension = entry_path
                        .extension()
                        .map(|e| e.to_string_lossy().to_string())
                        .unwrap_or_default();
                    (get_file_type(&extension), entry.metadata().map(|m| m.len()).unwrap_or(0))
                };
                
                files.push(FileInfo {
                    path: entry_path.to_string_lossy().to_string(),
                    name: file_name,
                    file_type,
                    size,
                    is_directory: is_dir,
                });
            }
        }
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    }
    
    // Sort: folders first, then files, both alphabetically
    files.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
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
        
        match trash::delete(path) {
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
            list_directory_tree,
            get_folder_contents_count,
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

