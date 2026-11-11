//! File system management functions
//! WARNING: please consider using the functions in `std::fs` instead.

use std::ffi::{CStr, CString, c_void};

/// Load file data as byte array (read), returns `None` if file failed to be read.
/// WARNING: please prefer using functions in `std::fs`, these do not check for errors.
pub fn load_file_data(file_name: &str) -> Option<Vec<u8>> {
    let file_name = CString::new(file_name).expect("file name should be valid utf8");

    let mut data_size = 0;
    let data_raw = unsafe { raylib_ffi::LoadFileData(file_name.as_ptr(), &mut data_size) };

    if data_raw.is_null() || data_size < 0 {
        return None;
    }

    // construct vec
    let data = unsafe { Vec::from_raw_parts(data_raw, data_size as usize, data_size as usize) };

    unsafe {
        raylib_ffi::UnloadFileData(data_raw);
    }

    Some(data)
}

/// Save data to file from byte array (write), returns true on success
/// WARNING: please prefer using functions in `std::fs`
pub fn save_file_data(file_name: &str, data: &[u8]) -> bool {
    let file_name_c = CString::new(file_name).expect("file name should be valid utf8");

    unsafe {
        raylib_ffi::SaveFileData(
            file_name_c.as_ptr(),
            data.as_ptr() as *mut c_void,
            data.len() as i32,
        )
    }
}

/// Export data to code (.h), returns true on success
pub fn export_data_as_code(data: &[u8], file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("file name should be valid utf8");

    unsafe { raylib_ffi::ExportDataAsCode(data.as_ptr(), data.len() as i32, file_name_c.as_ptr()) }
}

/// Load text data from file (read), returns `None` if file failed to be read.
pub fn load_file_text(file_name: &str) -> Option<String> {
    let file_name_c = CString::new(file_name).expect("file name should be valid utf8");

    let file_text_raw = unsafe { raylib_ffi::LoadFileText(file_name_c.as_ptr()) };

    if file_text_raw.is_null() {
        return None;
    }

    let file_text = unsafe { CStr::from_ptr(file_text_raw) }
        .to_string_lossy()
        .into_owned();

    unsafe {
        raylib_ffi::UnloadFileText(file_text_raw);
    }

    Some(file_text)
}

/// Save text data to file (write), returns true on success
/// WARNING: please prefer using functions in `std::fs`
pub fn save_file_text(file_name: &str, text: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("file name should be valid utf8");
    let text_c = CString::new(text).expect("text should be valid utf8");

    unsafe { raylib_ffi::SaveFileText(file_name_c.as_ptr(), text_c.as_ptr() as *mut i8) }
}

// File access custom callbacks
// WARNING: Callbacks setup is intended for advanced users

pub use raylib_ffi::{
    LoadFileDataCallback, LoadFileTextCallback, SaveFileDataCallback, SaveFileTextCallback,
};

/// Set custom file binary data loader
pub fn set_load_file_data_callback(callback: LoadFileDataCallback) {
    unsafe { raylib_ffi::SetLoadFileDataCallback(callback) }
}

/// Set custom file binary data saver
pub fn set_save_file_data_callback(callback: SaveFileDataCallback) {
    unsafe { raylib_ffi::SetSaveFileDataCallback(callback) }
}

/// Set custom file text data loader
pub fn set_load_file_text_callback(callback: LoadFileTextCallback) {
    unsafe { raylib_ffi::SetLoadFileTextCallback(callback) }
}

/// Set custom file text data saver
pub fn set_save_file_text_callback(callback: SaveFileTextCallback) {
    unsafe { raylib_ffi::SetSaveFileTextCallback(callback) }
}

/* TODO

int FileRename(const char *fileName, const char *fileRename); // Rename file (if exists), returns 0 on success
int FileRemove(const char *fileName);                         // Remove file (if exists), returns 0 on success
int FileCopy(const char *srcPath, const char *dstPath);       // Copy file from one path to another, dstPath created if it doesn't exist, returns 0 on success
int FileMove(const char *srcPath, const char *dstPath);       // Move file from one directory to another, dstPath created if it doesn't exist, returns 0 on success
int FileTextReplace(const char *fileName, const char *search, const char *replacement); // Replace text in an existing file, returns 0 on success
int FileTextFindIndex(const char *fileName, const char *search); // Find text in existing file, returns -1 if index not found or index otherwise
bool FileExists(const char *fileName);                        // Check if file exists
bool DirectoryExists(const char *dirPath);                    // Check if directory path exists
bool IsFileExtension(const char *fileName, const char *ext);  // Check file extension (recommended include point: .png, .wav)
bool IsFileHidden(const char *filePath);                      // Check if file path (file or directory) is hidden by OS
int GetFileLength(const char *fileName);                      // Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
long GetFileModTime(const char *fileName);                    // Get file modification time (last write time)
const char *GetFileExtension(const char *fileName);           // Get pointer to extension for a filename string (includes dot: '.png')
const char *GetFileName(const char *filePath);                // Get pointer to filename for a path string
const char *GetFileNameWithoutExt(const char *filePath);      // Get filename string without extension (uses static string)
const char *GetDirectoryPath(const char *filePath);           // Get full path for a provided fileName with path (uses static string)
const char *GetPrevDirectoryPath(const char *dirPath);        // Get previous directory path for a provided path (uses static string)
const char *GetWorkingDirectory(void);                        // Get current working directory (uses static string)
const char *GetApplicationDirectory(void);                    // Get the directory of the running application (uses static string)
int MakeDirectory(const char *dirPath);                       // Create directories (including full path requested), returns 0 on success
int ChangeDirectory(const char *dirPath);                     // Change working directory, returns 0 on success
bool IsPathFile(const char *path);                            // Check if provided path points to a file
bool IsPathDirectory(const char *path);                       // Check if provided path points to a directory
bool IsPathAbsolute(const char *path);                        // Check if provided path is an absolute path
bool IsFileNameValid(const char *fileName);                   // Check if fileName is valid for the platform/OS
FilePathList LoadDirectoryFiles(const char *dirPath);         // Load directory filepaths, files and directories, no subdirs scan
FilePathList LoadDirectoryFilesEx(const char *basePath, const char *filter, bool scanSubdirs); // Load directory filepaths with extension filtering and subdir scan; some filters available: '*.*','FILES*','DIRS*'
void UnloadDirectoryFiles(FilePathList files);                // Unload filepaths
bool IsFileDropped(void);                                     // Check if file has been dropped into window
FilePathList LoadDroppedFiles(void);                          // Load dropped filepaths
void UnloadDroppedFiles(FilePathList files);                  // Unload dropped filepaths
unsigned int GetDirectoryFileCount(const char *dirPath);      // Get the file count in a directory
unsigned int GetDirectoryFileCountEx(const char *basePath, const char *filter, bool scanSubdirs); // Get the file count in a directory with extension filtering and recursive directory scan. Use 'DIR' in the filter string to include directories in the result
*/
