#ifndef LIB_H_
#define LIB_H_

#include <stdint.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

typedef struct Slice Slice;
struct Slice {
  i64 *buf;
  i64 len;
  i64 cap;
};

Slice *slice_create(void);
void slice_destroy(Slice *);

void slice_add(Slice *, i64);
void slice_sort(Slice *);

#endif // LIB_H_
