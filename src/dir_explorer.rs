//

/// .
use std::fs;





// A função para listar os arquivos
pub fn list_files(dir_path: &str, file_type: Option<&str>, show_size: bool) {
    // Verifica se o tipo de arquivo foi especificado
    match file_type {
        Some(ft) => {
            println!("Listando arquivos de tipo '{}':", ft);
            let extensions = if !ft.is_empty() { vec![ft] } else { Vec::new() };
            if let Err(e) = explore_dir(dir_path, &extensions, show_size) {
                println!("Erro ao explorar diretório: {}", e);
                return; // Retorna imediatamente em caso de erro
            }
        }
        None => {
            println!("Listando todos os arquivos:");
            if let Err(e) = explore_dir(dir_path, &[], show_size) {
                println!("Erro ao explorar diretório: {}", e);
                return; // Retorna imediatamente em caso de erro
            }
        }
    }
}

// Função para explorar o diretório e listar arquivos
fn explore_dir(file_path: &str, file_extensions: &[&str], show_size: bool) -> Result<(), std::io::Error> {
    if file_extensions.is_empty() {
        show_files_and_size(file_path, show_size)?;
    } else {
        show_files_and_size_of_a_type(file_path, show_size, file_extensions)?;
    }
    Ok(())
}



// /// Explore o diretório especificado e exclua arquivos com extensões não desejadas.
// pub fn explore_dir(file_path: &str, file_extensions: &[&str]) -> Result<(), std::io::Error> {
//     // Ler o conteúdo do diretório
//     let paths = fs::read_dir(file_path)?;
//     if file_extensions.is_empty() {
//        // retorna todos os arquivos
//          show_files_and_size(file_path)?;
//     }
//     // Iterar sobre os arquivos no diretório
//     for path in paths {
//         let path = path?.path();
//         let file_name = path.file_name().unwrap().to_string_lossy().to_string(); // Converter para string
//         if is_file_extension_valid(&file_name, file_extensions) {
//            show_files_and_size(file_path)?;
            
//         }
//     }
//     Ok(())
// }






fn is_file_extension_valid(file_name: &str, file_extensions: &[&str]) -> bool {
    let extension = file_name.split('.').last().unwrap_or("");
    file_extensions.contains(&extension)
}



fn show_files_and_size(file_path: &str,show_size: bool) -> Result<(), std::io::Error> {
    let files = get_files(file_path)?;

    let is_show_size = show_size;
    if is_show_size{
        for file_name in files {
            let size = file_size(&file_name)?;
            println!("Arquivo: {}, Tamanho: {} mb", file_name, size);
        }
    }
    

    let total_size = total_file_size(file_path)?;
    println!("Tamanho total: {} mb", total_size);
    Ok(())
}

fn show_files_and_size_of_a_type(file_path: &str, show_size: bool, file_extensions: &[&str]) -> Result<(), std::io::Error> {
    let files = get_files(file_path)?;
    
    for file_name in files {
        if is_file_of_type(&file_name, file_extensions) {
            println!("Arquivo: {}", file_name);
            if show_size {
                let size = file_size(&file_name)?;
                println!("Tamanho: {} mb", size);
            }
        }
    }

    let total_size = total_file_size(file_path)?;
    println!("Tamanho total: {} mb", total_size);
    
    Ok(())
}

fn is_file_of_type(file_name: &str, file_extensions: &[&str]) -> bool {
    let extension = match file_name.split('.').last() {
        Some(ext) => ext,
        None => return false,
    };
    
    file_extensions.contains(&extension)
}




// fn show_file_content(file_path: &str) -> Result<(), std::io::Error> {
//     let content = fs::read_to_string(file_path)?;
//     println!("{}", content);
//     Ok(())
// }

fn get_files(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let paths = fs::read_dir(file_path)?;
    let mut files = Vec::new();

    for path in paths {
        let path = path?.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        files.push(file_name);
    }
    Ok(files)
}

fn file_size(file_path: &str) -> Result<f32, std::io::Error> {
    let metadata = fs::metadata(file_path)?;
    let size = metadata.len();
    let size_mb = convert_bytes_to_mb(size);
    Ok(size_mb)
}

fn total_file_size(file_path: &str) -> Result<f32, std::io::Error> {
    let files = get_files(file_path)?;
    let mut total_size = 0.0;

    for file_name in &files {
        let size = file_size(file_name)?;
        total_size += size;
    }
    Ok(total_size)
}

fn convert_bytes_to_mb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0
}

fn get_most_large_file(file_path: &str) -> Result<String, std::io::Error> {
    let files = get_files(file_path)?;
    let mut largest_file = String::new();
    let mut largest_size = 0.0; // Change the type to f32

    for file_name in files {
        let size = file_size(&file_name)?;
        if size > largest_size {
            largest_size = size;
            largest_file = file_name;
        }
    }
    Ok(largest_file)
}

fn get_most_small_file(file_path: &str) -> Result<String, std::io::Error> {
    let files = get_files(file_path)?;
    let mut smallest_file = String::new();
    let mut smallest_size = std::f32::MAX; // Change the type to f32

    for file_name in files {
        let size = file_size(&file_name)?;
        if size < smallest_size {
            smallest_size = size;
            smallest_file = file_name;
        }
    }
    Ok(smallest_file)
}

fn get_number_of_files_max(file_path: &str,number: u32) -> Result<usize, std::io::Error> {
    let files = get_files(file_path)?;
    let mut number_of_files = 0;

    for file_name in files {
        let size = file_size(&file_name)?;
        if size > number as f32 {
            number_of_files += 1;
        }
    }
    Ok(number_of_files)
}


fn order_top_files(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let files = get_files(file_path)?;
    let mut files_and_sizes = Vec::new();

    for file_name in files {
        let size = file_size(&file_name)?;
        files_and_sizes.push((file_name, size));
    }

    files_and_sizes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_files: Vec<String> = files_and_sizes.iter().map(|(file, _)| file.clone()).collect();
    Ok(top_files)
}


fn order_bottom_files(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let files = get_files(file_path)?;
    let mut files_and_sizes = Vec::new();

    for file_name in files {
        let size = file_size(&file_name)?;
        files_and_sizes.push((file_name, size));
    }

    files_and_sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let bottom_files: Vec<String> = files_and_sizes.iter().map(|(file, _)| file.clone()).collect();
    Ok(bottom_files)
}




// fn total_file_size

