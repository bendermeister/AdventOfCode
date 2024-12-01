#include "lib.h"
#include <stdlib.h>

Slice *slice_create(void) {
  Slice *s = malloc(sizeof(*s));
  s->len = 0;
  s->cap = 16;
  s->buf = malloc(s->cap * sizeof(*s->buf));
  return s;
}

void slice_destroy(Slice *s) {
  if (!s) {
    return;
  }
  free(s->buf);
  free(s);
}

static void grow_if(Slice *s) {
  if (s->len >= s->cap) {
    s->cap <<= 1;
    s->buf = realloc(s->buf, s->cap * sizeof(*s->buf));
  }
}

void slice_add(Slice *s, i64 i) {
  grow_if(s);
  s->buf[s->len++] = i;
}

static int intcmp(const void *a, const void *b) {
  return *(i64 *)a - *(i64 *)b;
}

void slice_sort(Slice *s) { qsort(s->buf, s->len, sizeof(*s->buf), intcmp); }
