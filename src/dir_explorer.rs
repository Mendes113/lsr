//
/// .
use std::fs;
// A função para listar os arquivos
pub fn list_files(mut dir_path: &str, file_type: &str, order_by_size: &str, show_size: bool, delete: bool) {
    if dir_path.is_empty() {
       dir_path = ".";
    }

    print!("Diretório: {}\n", dir_path);
    print!("Tipo de arquivo: {}\n", file_type);
    print!("Ordenar por tamanho: {}\n", order_by_size);
    print!("Mostrar tamanho: {}\n", show_size);

    match file_type {
        file_type if !file_type.is_empty() => {
            println!("Listando arquivos de tipo '{}':", file_type);
            let extensions = vec![file_type];
            if(delete){
                if let Err(e) = delete_file_type(dir_path, &extensions) {
                    println!("Erro ao deletar arquivos: {}", e);
                    return; // Retorna imediatamente em caso de erro
                }
            }
            if let Err(e) = explore_dir(dir_path, &extensions, order_by_size, show_size) {
                println!("Erro ao explorar diretório: {}", e);
                return; // Retorna imediatamente em caso de erro
            }
        }
        _ => {
            println!("Listando todos os arquivos:");
            if let Err(e) = explore_dir(dir_path, &[], order_by_size, show_size) {
                println!("Erro ao explorar diretório: {}", e);
                return; // Retorna imediatamente em caso de erro
            }
        }
    }

}


// Função para explorar o diretório e listar arquivos
fn explore_dir(file_path: &str, file_extensions: &[&str],order_by_size: &str, show_size: bool) -> Result<(), std::io::Error> {


    if file_extensions.is_empty() {
        show_files_and_size(file_path, show_size, order_by_size)?;
    } else {
        show_files_and_size_of_a_type(file_path, show_size,order_by_size, file_extensions)?;
    }

    Ok(())
}
fn show_files_and_size(file_path: &str,show_size: bool, order_by_size: &str) -> Result<(), std::io::Error> {
    // let files = get_files(file_path)?;
   match order_by_size {
        "-b" => {
            let files = order_bottom_files(file_path)?;
            let is_show_size = show_size;
            if is_show_size{
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("Arquivo: {}, Tamanho: {} mb", file_name, size);

                }
            }
        }
        "-t" => {
            let files = order_top_files(file_path)?;
            let is_show_size = show_size;
            if is_show_size{
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("Arquivo: {}, Tamanho: {} mb", file_name, size);


                }
            }
        }
        _ => {
            let files = get_files(file_path)?;
            let is_show_size = show_size;
            if is_show_size{
                for file_name in files {
                    let size = file_size(&file_name)?;
                    println!("Arquivo: {}, Tamanho: {} mb", file_name, size);
                }
            }else {
                for file_name in files {
                    println!("Arquivo: {}", file_name);
                }
            }
        }
    }


    Ok(())
}
fn show_files_and_size_of_a_type(file_path: &str, show_size: bool, order_by_size: &str, file_extensions: &[&str]) -> Result<(), std::io::Error> {
    let is_show_size = show_size;
    let files = match order_by_size {
        "-b" => order_bottom_files(file_path)?,
        "-t" => order_top_files(file_path)?,
        _ => get_files(file_path)?,
    };
    for file_name in files {
        if is_file_of_type(&file_name, file_extensions) {
            if is_show_size {
                let size = file_size(&file_name)?;
                println!("Arquivo: {}, Tamanho: {} mb", file_name, size);
            } else {
                println!("Arquivo: {}", file_name);
            }
        }
    }
    Ok(())
}
fn is_file_of_type(file_name: &str, file_extensions: &[&str]) -> bool {
    if let Some(extension) = file_name.split('.').last() {
        return file_extensions.contains(&extension);
    }
    false
}

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
// fn get_most_large_file(file_path: &str) -> Result<String, std::io::Error> {
//     let files = get_files(file_path)?;
//     let mut largest_file = String::new();
//     let mut largest_size = 0.0; // Change the type to f32
//     for file_name in files {
//         let size = file_size(&file_name)?;
//         if size > largest_size {
//             largest_size = size;
//             largest_file = file_name;
//         }
//     }
//     Ok(largest_file)
// }
// fn get_most_small_file(file_path: &str) -> Result<String, std::io::Error> {
//     let files = get_files(file_path)?;
//     let mut smallest_file = String::new();
//     let mut smallest_size = std::f32::MAX; // Change the type to f32
//     for file_name in files {
//         let size = file_size(&file_name)?;
//         if size < smallest_size {
//             smallest_size = size;
//             smallest_file = file_name;
//         }
//     }
//     Ok(smallest_file)
// }
// fn get_number_of_files_max(file_path: &str,number: u32) -> Result<usize, std::io::Error> {
//     let files = get_files(file_path)?;
//     let mut number_of_files = 0;
//     for file_name in files {
//         let size = file_size(&file_name)?;
//         if size > number as f32 {
//             number_of_files += 1;
//         }
//     }
//     Ok(number_of_files)
// }
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

fn delete_file_type(file_path: &str, file_extensions: &[&str]) -> Result<(), std::io::Error> {
    let files = get_files(file_path)?;
    for file_name in files {
        if is_file_of_type(&file_name, file_extensions) {
            fs::remove_file(file_name)?;
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {

    use crate::dir_explorer::{file_size, get_files, order_bottom_files, order_top_files};


    #[test]
    fn test_file_size() {
        //log file size
        let size = file_size("das.jpg").unwrap();
        assert_eq!(size, 0.0); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_get_files() {
        //log file size
        let files = get_files("./").unwrap();
        print!("{:?}", files);
        assert_eq!(files.len(),10); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_order_top_files() {
        //log file size
        let files = order_top_files("./").unwrap();
        print!("{:?}", files);
       //verifica se o primeiro arquvivo é o maior
        assert_eq!(files[0],"lsr"); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_order_bottom_files() {
        //log file size
        let files = order_bottom_files("./").unwrap();
        print!("{:?}", files);
       //verifica se o primeiro arquvivo é o maior
        assert_eq!(files[0],"das.jpg"); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_most_large_file() {
        //log file size
        let files = order_top_files("./").unwrap();
        print!("{:?}", files);
       //verifica se o primeiro arquvivo é o maior
        assert_eq!(files[0],"lsr"); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_most_small_file() {
        //log file size
        let files = order_bottom_files("./").unwrap();
        print!("{:?}", files);
       //verifica se o primeiro arquvivo é o maior
        assert_eq!(files[0],"das.jpg"); // Substitua 0.0 pelo tamanho esperado do arquivo
    }

    #[test]
    fn test_is_file_of_type() {
        //log file size
        let files = get_files("./").unwrap();
        let is_file = crate::dir_explorer::is_file_of_type(&files[1], &["md"]);
        print!("{:?}", files);
        assert_eq!(is_file,true); // Substitua 0.0 pelo tamanho esperado do arquivo
    }


#[test]
fn test_total_file_size() {
    let size = crate::dir_explorer::total_file_size("./").unwrap();
    println!("{:?}", size);

    let expected_size = 0.4831;
    let tolerance = 0.1;
    let lower_bound = expected_size - tolerance;
    let upper_bound = expected_size + tolerance;
    print!("{:?}", size);
    assert!(size >= lower_bound && size <= upper_bound);
}


#[test]
fn test_show_files_and_size() {

    let result = crate::dir_explorer::show_files_and_size("./", true, "");

    // verify if the function ran successfully
    assert!(result.is_ok());
}


#[test]
fn test_show_files_and_size_of_a_type() {

    let result = crate::dir_explorer::show_files_and_size_of_a_type("./", true, "", &["toml"]);

    // verify if the function ran successfully
    assert!(result.is_ok());

}



#[test]
fn test_explore_dir() {

    let result = crate::dir_explorer::explore_dir("./", &["toml"], "", true);

    // verify if the function ran successfully
    assert!(result.is_ok());
}

//explore dir with no file type

#[test]
fn test_explore_dir_no_file_type() {

    let result = crate::dir_explorer::explore_dir("./", &[], "", true);

    // verify if the function ran successfully
    assert!(result.is_ok());

}

}