#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*) calloc ((size), sizeof (type)))

typedef struct potential_union_find {
  i32 *parent;
  i32 *weight;
  i32 size;
} union_find;

union_find* new_union_find (i32 n) {
  union_find *u = ALLOC (1, union_find);
  u->parent = ALLOC (n, i32);
  u->weight = ALLOC (n, i32);
  for (i32 i = 0; i < n; ++i) {
    u->parent[i] = -1;
    u->weight[i] = 0;
  }
  return u;
}

i32 root (union_find *u, i32 x) {
  while (u->parent[x] >= 0) {
    x = u->parent[x];
  }
  return x;
}

int same (union_find *u, i32 x, i32 y) {
  return root (u, x) == root (u, y);
}

i32 get_weight (union_find *u, i32 x) {
  i32 sum = 0;
  while (u->parent[x] >= 0) {
    sum += u->weight[x];
    x = u->parent[x];
  }
  return sum;
}

int unite (union_find *u, i32 x, i32 y, i32 z) {
  z += get_weight (u, x);
  z -= get_weight (u, y);
  x = root (u, x);
  y = root (u, y);
  if (x == y) {
    return z == 0;
  }
  if (u->parent[x] > u->parent[y]) {
    i32 swap = x;
    x = y;
    y = swap;
    z = -z;
  }
  u->parent[x] += u->parent[y];
  u->parent[y] = x;
  u->weight[y] = z;
  return 1;
}

void run (void) {
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  union_find *u = new_union_find (n);
  while (q--) {
    i32 op, x, y;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &op, &x, &y);
    if (op == 0) {
      i32 z;
      scanf ("%" SCNi32, &z);
      if (!unite (u, x, y, z)) {
        puts ("Contradiction");
        exit (1);
      }
    } else if (!same (u, x, y)) {
      puts ("?");
    } else {
      printf ("%" PRIi32 "\n", get_weight (u, y) - get_weight (u, x));
    }
  }
}

int main (void) {
  run();
  return 0;
}

