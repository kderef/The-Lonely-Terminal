use std::env;
use std::fs;
use raylib_bindings::rcore::filesystem::{load_file_text, save_file_text};

#[test]
fn test_save_and_load_file_text() {
    // Use a temporary file in the OS temp directory
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("raylib_test_temp.txt");
    let content = "Hello, world!";

    // Ensure we start clean
    let _ = fs::remove_file(&file_path);

    // Write the text file using our bindings
    assert!(save_file_text(file_path.to_str().unwrap(), content), "Failed to save file");

    // Read it back
    let loaded = load_file_text(file_path.to_str().unwrap()).expect("Failed to load file");
    assert_eq!(content, loaded, "Loaded text does not match written content");

    // Clean up the temporary file
    fs::remove_file(file_path).ok();
}
