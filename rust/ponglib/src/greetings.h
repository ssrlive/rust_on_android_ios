#if !defined(__RUST_GREETING_H__)
#define __RUST_GREETING_H__

#include <stdint.h>

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);

#endif // __RUST_GREETING_H__
