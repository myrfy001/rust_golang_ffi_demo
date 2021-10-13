#include "stdint.h"

uintptr_t simple_rust_func_called_from_go(uint8_t arg1, uint16_t arg2, uint32_t arg3);

char* receive_str_and_return_string(char*);
char* receive_string_and_return_string(char*);
char* receive_str_and_return_str(char*);
// the follow line is a very ugly api design, only used as example, never use in real code.
char* receive_string_and_return_str(char*, char**, char**, uintptr_t*, uintptr_t*);

void free_string_alloc_by_rust_by_raw_parts(char**, uintptr_t, uintptr_t);
void free_cstring_alloc_by_rust(char*);