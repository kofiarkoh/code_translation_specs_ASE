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

typedef struct node_full {
  i64 val;
  i64 val2;
  i32 index;
} node;

int cmp_node (const void *a, const void *b) {
  i64 d = ((node *)a)->val - ((node *)b)->val;
  if (d != 0) return d > 0 ? -1 : 1;
  d = ((node *)a)->val2 - ((node *)b)->val2;
  return d < 0 ? -1 : 1;
}

void run (void) {
  i32 n, x;
  scanf ("%" SCNi32 "%" SCNi32, &n, &x);
  i32 *b = (i32 *) calloc (n, sizeof (i32));
  i32 *l = (i32 *) calloc (n, sizeof (i32));
  i32 *u = (i32 *) calloc (n, sizeof (i32));
  i64 ini = 0;
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, b + i, l + i, u + i);
    ini -= (i64) l[i] * b[i];
  }
  uint8_t *used = (uint8_t *) calloc (n, sizeof (uint8_t));
  node *p = (node *) calloc (n, sizeof (node));
  i64 left = -1;
  i64 right = (i64) x * n;
  while (right - left > 1) {
    i64 m = (left + right) / 2;
    i64 d = m % x;
    memset (used, 0, sizeof (uint8_t) * n);
    for (i32 i = 0; i < n; ++i) {
      p[i] = (node) {(i64) u[i] * (x - b[i]) + (i64) l[i] * b[i], d <= m ? (i64) d * l[i] : (i64) u[i] * (d - b[i]) + (i64) l[i] * b[i], i};
    }
    qsort (p, n, sizeof (node), cmp_node);
    i64 sum = ini;
    for (i32 i = 0; m >= x; ++i, m -= x) {
      sum += p[i].val;
      used[p[i].index] = 1;
    }
    i64 max = 0;
    for (i32 i = 0; i < n; ++i) {
      if (used[i]) continue;
      if (m <= b[i]) {
	max = MAX(max, (i64) l[i] * m);
      } else {
	max = MAX(max, (i64) l[i] * b[i] + (i64) u[i] * (m - b[i]));
      }
    }
    if (sum + max >= 0) {
      right = (left + right) / 2;
    } else {
      left = (left + right) / 2;
    }
  }
  printf ("%" PRIi64 "\n", right);
}

int main (void) {
  run();
  return 0;
}
