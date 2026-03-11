#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

typedef i32 RUQ_val;

typedef struct range_update_query {
  RUQ_val *val;
  int32_t size;
  int32_t bit;
  RUQ_val empty;
} RUQ;

RUQ* new_RUQ (const int32_t n) {
  int32_t k = 0;
  while ((1 << k) < n) ++k;
  RUQ *s = (RUQ *) calloc (1, sizeof (RUQ));
  s->val = (RUQ_val *) calloc (2 << k, sizeof (RUQ_val));
  s->size = 1 << k;
  s->bit = k;
  s->empty = -2;
  s->val[1] = -1;
  return s;
}

static inline void propagate (RUQ *s, int32_t k) {
  if (s->val[k] == s->empty) return;
  s->val[2 * k] = s->val[2 * k + 1] = s->val[k];
  s->val[k] = s->empty;
}

void update (RUQ *s, int32_t l, int32_t r, RUQ_val v) {
  for (int32_t i = s->bit; i > 0; --i) {
    propagate (s, (l + s->size) >> i);
    propagate (s, (r - 1 + s->size) >> i);
  }
  for (l += s->size, r += s->size; l < r; l >>= 1, r >>= 1) {
    if (l & 1) s->val[l++] = v;
    if (r & 1) s->val[--r] = v;
  }
}

RUQ_val find (RUQ *s, int32_t x) {
  x += s->size;
  for (int32_t i = s->bit; i >= 0; --i) {
    if (s->val[x >> i] != s->empty) {
      return s->val[x >> i];
    }
  }
  return s->empty;
}

typedef struct node {
  i32 s, t, x;
} node;

int cmp_node (const void *a, const void *b) {
  i32 d = ((node *)a)->x - ((node *)b)->x;
  return d == 0 ? 0 : d > 0 ? -1 : 1;
}

int cmp_int (const void *a, const void *b) {
  i32 d = *(i32 *)a - *(i32 *)b;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

i32 to_index (i32 *z, i32 len, i32 v) {
  i32 l = 0;
  i32 r = len;
  while (r - l > 1) {
    i32 m = (l + r) / 2;
    if (z[m] <= v) {
      l = m;
    } else {
      r = m;
    }
  }
  return l;
}

void run (void) {
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  node *p = (node *) calloc (n, sizeof (node));
  i32 *d = (i32 *) calloc (q, sizeof (i32));
  i32 *z = (i32 *) calloc (2 * n + q, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    i32 s, t, x;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &s, &t, &x);
    p[i] = (node) {s - x, t - x, x};
    z[i] = s - x;
    z[i + n] = t - x;
  }
  for (i32 i = 0; i < q; ++i) {
    scanf ("%" SCNi32, d + i);
    z[2 * n + i] = d[i];
  }
  qsort (p, n, sizeof (node), cmp_node);
  qsort (z, 2 * n + q, sizeof (i32), cmp_int);
  i32 len = 1;
  for (i32 i = 1; i < 2 * n + q; ++i) {
    if (z[i] == z[len - 1]) continue;
    z[len++] = z[i];
  }
  RUQ *s = new_RUQ (len);
  for (i32 i = 0; i < n; ++i) {
    i32 l = to_index (z, len, p[i].s);
    i32 r = to_index (z, len, p[i].t);
    update (s, l, r, p[i].x);
  }
  for (i32 i = 0; i < q; ++i) {
    i32 k = to_index (z, len, d[i]);
    i32 v = find (s, k);
    printf ("%" PRIi32 "\n", v);
  }
}

int main (void) {
  run();
  return 0;
}
