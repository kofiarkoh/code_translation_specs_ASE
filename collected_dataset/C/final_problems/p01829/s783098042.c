#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

#define MAX(a,b) ((a) > (b) ? (a) : (b))
#define MIN(a,b) ((a) < (b) ? (a) : (b))
#define ABS(a) ((a) > (0) ? (a) : -(a))
#define ALLOC(size,type) ((type*) calloc ((size), sizeof (type)))

const i32 f = 10;

i64 max_diff;
i64 max_val;
void calc (i32 k, i32 *a, i64 v, i32 len) {
  if (k == f - 1) {
    i64 t = 0;
    i64 p = 1;
    for (i32 i = 0; i < len; ++i) {
      t = 10 * t + a[i];
      p *= 10;
    }
    i64 diff = MIN (ABS (t - v), p - ABS(t - v));
    if (diff > max_diff || (diff == max_diff && max_val > t)) {
      max_diff = diff;
      max_val = t;
    }
    return;
  }
  calc (k + 1, a, v, len);
  for (i32 i = k + 1; i < f; ++i) {
    i32 swap = a[k];
    a[k] = a[i];
    a[i] = swap;
    calc (k + 1, a, v, len);
  }
  i32 q = a[k];
  for (i32 i = k + 1; i < f; ++i) {
    a[i - 1] = a[i];
  }
  a[f - 1] = q;
}

void run (void) {
  char s[11];
  scanf ("%s", s);
  i32 len = strlen (s);
  i64 v = 0;
  for (i32 i = 0; i < len; ++i) {
    v = 10 * v + s[i] - '0';
  }
  i32 a[10];
  for (i32 i = 0; i < f; ++i) {
    a[i] = i;
  }
  max_diff = 0;
  max_val = 1000000000000;
  calc (0, a, v, len);
  i64 p = 1;
  for (i32 i = 1; i < len; ++i) {
    p *= 10;
  }
  if (max_val < p) putchar ('0');
  printf ("%" PRIi64 "\n", max_val);
}

int main (void) {
  run();
  return 0;
}

