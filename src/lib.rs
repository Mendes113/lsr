use colored::*;
use std::fs;
pub fn list_files(
    mut dir_path: &str,
    file_type: &str,
    order_by_size: &str,
    show_size: bool,
    delete: bool,
    recursive: bool,
) {
    if dir_path.is_empty() {
        dir_path = ".";
    }
    match file_type {
        file_type if !file_type.is_empty() => {
            println!("Listing files by type'{}':", file_type);
            let extensions = vec![file_type];
            if delete {
                if let Err(e) = delete_file_type(dir_path, &extensions) {
                    println!("Error deleting files: {}", e);
                    return;
                }
            }
            if let Err(e) = explore_dir(dir_path, &extensions, order_by_size, show_size, recursive)
            {
                println!("Error Exploring directories : {}", e);
                return;
            }
        }
        _ => {
            println!("Listing all files:");
            if let Err(e) = explore_dir(dir_path, &[], order_by_size, show_size, recursive) {
                println!("Error exploring directories: {}", e);
                return;
            }
        }
    }
}

pub fn explore_dir(
    file_path: &str,
    file_extensions: &[&str],
    order_by_size: &str,
    show_size: bool,
    recursive: bool,
) -> Result<(), std::io::Error> {
    if file_extensions.is_empty() {
        show_files_and_size(file_path, show_size, order_by_size, recursive)?;
    } else {
        show_files_and_size_of_a_type(
            file_path,
            show_size,
            order_by_size,
            file_extensions,
            recursive,
        )?;
    }
    Ok(())
}

pub fn show_files_and_size(
    file_path: &str,
    show_size: bool,
    order_by_size: &str,
    recursive: bool,
) -> Result<(), std::io::Error> {
    match order_by_size {
        "-b" => {
            let files = order_bottom_files(file_path, recursive)?;
            let is_show_size = show_size;
            if is_show_size {
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("{},{} mb", formated_file_or_dir(&file_name), formated_color_by_size(size));
                }
            }
        }
        "-t" => {
            let files = order_top_files(file_path, recursive)?;
            let is_show_size = show_size;
            if is_show_size {
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("{},  {} mb", formated_file_or_dir(&file_name), formated_color_by_size(size));
                }
            }
        }
        _ => {
            let files = get_files(file_path, recursive)?;
            let is_show_size = show_size;
            if is_show_size {
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("{}, {} mb", formated_file_or_dir(&file_name), formated_color_by_size(size));
                }
            } else {
                for file_name in files {
                    println!("{}", formated_file_or_dir(&file_name));
                }
            }
        }
    }
    Ok(())
}

pub fn recursive_dir_explorer(file_path: &str) -> Result<(), std::io::Error> {
    let paths = fs::read_dir(file_path)?;
    for path in paths {
        let path = path?.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        if path.is_dir() {
            if path.starts_with(".") {
                continue;
            }
            recursive_dir_explorer(&path.to_string_lossy())?;
        } else {
            println!("{}", formated_file_or_dir(&file_name));
        }
    }
    Ok(())
}

pub fn show_files_and_size_of_a_type(
    file_path: &str,
    show_size: bool,
    order_by_size: &str,
    file_extensions: &[&str],
    recursive: bool,
) -> Result<(), std::io::Error> {
    let is_show_size = show_size;
    let files = match order_by_size {
        "-b" => order_bottom_files(file_path, recursive)?,
        "-t" => order_top_files(file_path, recursive)?,
        _ => get_files(file_path, recursive)?,
    };
    for file_name in files {
        if is_file_of_type(&file_name, file_extensions) {
            if is_show_size {
                let size = file_size(&file_name)?;
                println!("{}, {} mb", formated_file_or_dir(&file_name), formated_color_by_size(size));
            } else {
                println!(" {}", formated_file_or_dir(&file_name));
            }
        }
    }
    Ok(())
}

pub fn is_file_of_type(file_name: &str, file_extensions: &[&str]) -> bool {
    if let Some(extension) = file_name.split('.').last() {
        return file_extensions.contains(&extension);
    }
    false
}

pub fn get_files(file_path: &str, recursive: bool) -> Result<Vec<String>, std::io::Error> {
    let paths = fs::read_dir(file_path)?;
    let mut files = Vec::new();
    for path in paths {
        let path = path?.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        if recursive && path.is_dir() {
            recursive_dir_explorer(&path.to_string_lossy())?;
        } else {
            files.push(file_name);
        }
    }
    Ok(files)
}

pub fn file_size(file_path: &str) -> Result<f32, std::io::Error> {
    let metadata = fs::metadata(file_path)?;
    let size = metadata.len();
    let size_mb = convert_bytes_to_mb(size);
    Ok(size_mb)
}

pub fn convert_bytes_to_mb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0
}

pub fn order_top_files(file_path: &str, recursive: bool) -> Result<Vec<String>, std::io::Error> {
    let files = get_files(file_path, recursive)?;
    let mut files_and_sizes = Vec::new();
    for file_name in files {
        let size = file_size(&file_name)?;
        files_and_sizes.push((file_name, size));
    }
    files_and_sizes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_files: Vec<String> = files_and_sizes
        .iter()
        .map(|(file, _)| file.clone())
        .collect();
    Ok(top_files)
}

pub fn order_bottom_files(file_path: &str, recursive: bool) -> Result<Vec<String>, std::io::Error> {
    let files = get_files(file_path, recursive)?;
    let mut files_and_sizes = Vec::new();
    for file_name in files {
        let size = file_size(&file_name)?;
        files_and_sizes.push((file_name, size));
    }
    files_and_sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let bottom_files: Vec<String> = files_and_sizes
        .iter()
        .map(|(file, _)| file.clone())
        .collect();
    Ok(bottom_files)
}

pub fn delete_file_type(file_path: &str, file_extensions: &[&str]) -> Result<(), std::io::Error> {
    let recursive = false;
    let files = get_files(file_path, recursive)?;
    for file_name in files {
        if is_file_of_type(&file_name, file_extensions) {
            fs::remove_file(file_name)?;
        }
    }
    Ok(())
}

fn formated_color_by_size(size: f32) -> String {
    let size_str = convert_to_string(size); // Convertendo o tamanho para uma string
    if size > 100.0 {
        size_str.red().to_string()
    } else if size > 50.0 {
        size_str.yellow().to_string()
    } else if size > 1.0 {
        size_str.blue().to_string()
    } else {
        size_str.green().to_string()
    }
}


fn formated_file_or_dir(file_name: &str) -> String {
    if file_name.ends_with("/") || file_name.starts_with(".") {
        file_name.bright_purple().to_string() 
    } else {
        file_name.green().to_string()
    }
}

fn convert_to_string(size: f32) -> String {
    size.to_string()
}