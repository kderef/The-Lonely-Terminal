//! Compression/Encoding functionality

/* TODO
unsigned char *CompressData(const unsigned char *data, int dataSize, int *compDataSize);        // Compress data (DEFLATE algorithm), memory must be MemFree()
unsigned char *DecompressData(const unsigned char *compData, int compDataSize, int *dataSize);  // Decompress data (DEFLATE algorithm), memory must be MemFree()
char *EncodeDataBase64(const unsigned char *data, int dataSize, int *outputSize);               // Encode data to Base64 string (includes NULL terminator), memory must be MemFree()
unsigned char *DecodeDataBase64(const char *text, int *outputSize);                             // Decode Base64 string (expected NULL terminated), memory must be MemFree()
unsigned int ComputeCRC32(const unsigned char *data, int dataSize); // Compute CRC32 hash code
unsigned int *ComputeMD5(const unsigned char *data, int dataSize);  // Compute MD5 hash code, returns static int[4] (16 bytes)
unsigned int *ComputeSHA1(const unsigned char *data, int dataSize); // Compute SHA1 hash code, returns static int[5] (20 bytes)
unsigned int *ComputeSHA256(const unsigned char *data, int dataSize); // Compute SHA256 hash code, returns static int[8] (32 bytes)
*/
