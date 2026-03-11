#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))
#define SORT(a,num,cmp) qsort((a),(num),sizeof(*(a)),cmp)

typedef struct work {
  i32 a, b;
} node;

int cmp_node (const void *a, const void *b) {
  i32 d = ((node *)a)->b - ((node *)b)->b;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  node *p = ALLOC (n, node);
  for (i32 i = 0; i < n; ++i) {
    i32 a, b;
    scanf ("%" SCNi32 "%" SCNi32, &a, &b);
    p[i] = (node) {a, b};
  }
  SORT (p, n, cmp_node);
  i32 sum = 0;
  for (i32 i = 0; i < n; ++i) {
    sum += p[i].a;
    if (sum > p[i].b) {
      puts ("No");
      return;
    }
  }
  puts ("Yes");
}

int main (void) {
  run();
  return 0;
}
