//! File system management functions
/* TODO
unsigned char *LoadFileData(const char *fileName, int *dataSize); // Load file data as byte array (read)
void UnloadFileData(unsigned char *data);                     // Unload file data allocated by LoadFileData()
bool SaveFileData(const char *fileName, const void *data, int dataSize); // Save data to file from byte array (write), returns true on success
bool ExportDataAsCode(const unsigned char *data, int dataSize, const char *fileName); // Export data to code (.h), returns true on success
char *LoadFileText(const char *fileName);                     // Load text data from file (read), returns a '\0' terminated string
void UnloadFileText(char *text);                              // Unload file text data allocated by LoadFileText()
bool SaveFileText(const char *fileName, const char *text);    // Save text data to file (write), string must be '\0' terminated, returns true on success

// File access custom callbacks
// WARNING: Callbacks setup is intended for advanced users
void SetLoadFileDataCallback(LoadFileDataCallback callback);  // Set custom file binary data loader
void SetSaveFileDataCallback(SaveFileDataCallback callback);  // Set custom file binary data saver
void SetLoadFileTextCallback(LoadFileTextCallback callback);  // Set custom file text data loader
void SetSaveFileTextCallback(SaveFileTextCallback callback);  // Set custom file text data saver

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
