#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

typedef struct node {
  i32 s, t;
} node;

int cmp (const void *a, const void *b) {
  i32 d = ((node *)a)->t - ((node *)b)->t;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  node *p = (node *) calloc (n, sizeof (node));
  for (i32 i = 0; i < n; ++i) {
    i32 s, t;
    scanf ("%" SCNi32 "%" SCNi32, &s, &t);
    p[i] = (node) {s, t};
  }
  qsort (p, n, sizeof (node), cmp);
  i32 ans = 0;
  i32 now = 0;
  for (i32 i = 0; i < n; ++i) {
    if (now < p[i].s) {
      now = p[i].t;
      ans++;
    }
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}
