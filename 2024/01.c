#include "lib.h"
#include <assert.h>
#include <stdio.h>

i64 distance(i64 a, i64 b) {
  i64 c = a - b;
  return c < 0 ? -c : c;
}

int main(void) {
  Slice *left = slice_create();
  Slice *right = slice_create();

  i64 l;
  i64 r;

  while (scanf("%ld %ld", &l, &r) != EOF) {
    slice_add(left, l);
    slice_add(right, r);
  }

  assert(left->len == right->len);

  slice_sort(left);
  slice_sort(right);

  i64 sum = 0;

  for (i64 i = 0; i < left->len; i += 1) {
    sum += distance(left->buf[i], right->buf[i]);
  }
  printf("Part One: %ld\n", sum);

  i64 score = 0;
  for (i64 i = 0; i < left->len; i += 1) {
    for (i64 j = 0; j < right->len; j += 1) {
      if (left->buf[i] == right->buf[j]) {
        score += left->buf[i];
      }
    }
  }

  printf("Part Two: %ld\n", score);

  slice_destroy(left);
  slice_destroy(right);
  return 0;
}
