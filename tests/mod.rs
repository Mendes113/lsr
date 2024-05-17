use std::fs::{self, File};
use std::io::Write;
use lsr::*; // Substitua pelo nome do seu pacote

fn setup_test_dir() -> String {
    let test_dir = "test_dir";
    let _ = fs::create_dir(test_dir);
    test_dir.to_string()
}

fn create_test_file(file_path: &str, size: usize) {
    let mut file = File::create(file_path).unwrap();
    let data = vec![0u8; size];
    file.write_all(&data).unwrap();
}

#[test]
fn test_is_file_of_type() {
    assert!(is_file_of_type("file.txt", &["txt"]));
    assert!(!is_file_of_type("file.txt", &["rs"]));
}

#[test]
fn test_file_size() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file.txt", test_dir);
    create_test_file(&file_path, 1024);
    let size = file_size(&file_path).unwrap();
    assert_eq!(size, 1.0);
    let _ = fs::remove_file(file_path);
    let _ = fs::remove_dir(test_dir);
}

#[test]
fn test_get_files() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file", test_dir);
    create_test_file(&file_path, 1024);

    let files = get_files(&test_dir, false).unwrap();
    assert!(files.contains(&"test_file.txt".to_string()));

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_dir(test_dir);
    assert!(get_files("non_existent_dir", false).is_err());
}


#[test]
fn test_show_files_and_size_of_a_type() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file.txt", test_dir);
    create_test_file(&file_path, 1024);

    let file_extensions = vec!["txt"];
    let _ = show_files_and_size_of_a_type(&test_dir, true, "", &file_extensions, false);

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_dir(test_dir);
}

#[test]
fn test_get_size() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file.txt", test_dir);
    create_test_file(&file_path, 1024);

    let size = file_size(&test_dir).unwrap();
    assert_eq!(size, 0.00390625);
}

#[test]
fn test_recursive_dir_explorer() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file.txt", test_dir);
    create_test_file(&file_path, 1024);

    let _ = recursive_dir_explorer(&test_dir);

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_dir(test_dir);
}

// #[test]
// fn test_order_top_files() {
//     let test_dir = setup_test_dir();
//     let file_path = format!("{}/test_file.txt", test_dir);
//     create_test_file(&file_path, 1024);

//     let files_result = order_top_files(&test_dir, false);
//     if let Ok(files) = files_result {
//         assert_eq!(files.len(), 1);
//     } else {
//         panic!("Failed to get files: {:?}", files_result);
//     }

//     let _ = fs::remove_file(file_path);
//     let _ = fs::remove_dir(test_dir);
// }

#[test]
fn test_list_files() {
    let test_dir = setup_test_dir();
    let file_path = format!("{}/test_file.txt", test_dir);
    create_test_file(&file_path, 1024);

    let _ = list_files(&test_dir, "", "", false, false, false);

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_dir(test_dir);
}