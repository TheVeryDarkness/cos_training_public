/* Helper for getting init_calls_[start/end] */
#include <stdint.h>

typedef uint64_t u64;

__attribute__((__section__(".initcall5.init"))) //
extern void *volatile init_calls_start;
__attribute__((__section__(".initcall5.init"))) //
extern void *volatile init_calls_end;

u64 initcalls_start() { return (u64)&init_calls_start; }

u64 initcalls_end() { return (u64)&init_calls_end; }
