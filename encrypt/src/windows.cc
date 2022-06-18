// #pragma comment(lib, "crypt32.lib")

#include <stdint.h>
#include <stdio.h>
#include <windows.h>
// header sequence
#include <Wincrypt.h>

typedef struct CryptResult {
  uint8_t* data;
  uint32_t len;
} CryptResult;

extern "C" CryptResult win_crypt_protect(const uint8_t* data, uint32_t len,
                                         bool protect) {
  DATA_BLOB DataIn;
  DATA_BLOB DataOut;
  CryptResult result = {0};

  DataIn.pbData = (BYTE*)data;
  DataIn.cbData = len;

  if (protect) {
    if (!CryptProtectData(&DataIn, NULL, NULL, NULL, NULL, 0, &DataOut)) {
      fprintf(stderr, "CryptProtectData error!\n");
      return result;
    }
  } else {
    if (!CryptUnprotectData(&DataIn, NULL, NULL, NULL, NULL, 0, &DataOut)) {
      fprintf(stderr, "CryptUnprotectData error!");
      return result;
    }
  }
  result.data = DataOut.pbData;
  result.len = DataOut.cbData;
  return result;
}

extern "C" void win_crypt_free(void* data) {
  if (data) LocalFree(data);
}
