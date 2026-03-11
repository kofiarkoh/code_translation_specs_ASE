#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct quick_find{
  i32 *id;
  i32 **elem;
  i32 *size;
  i32 *max_size;
  i32 num;
} quick_find;

void init_quick_find (quick_find * const q) {
  for (i32 i = 0; i < q->num; ++i) {
    q->id[i] = i;
    free (q->elem[i]);
    q->elem[i] = (i32 *) calloc (1, sizeof (i32));
    q->elem[i][0] = i;
    q->size[i] = 1;
    q->max_size[i] = 1;
  }
}

quick_find* new_quick_find (const i32 n) {
  quick_find *q = (quick_find *) calloc (1, sizeof (quick_find));
  q->id = (i32 *) calloc (3 * n, sizeof (i32));
  q->elem = (i32 **) calloc (n, sizeof (i32 *));
  q->size = q->id + n;
  q->max_size = q->size + n;
  q->num = n;
  init_quick_find (q);
  return q;
}

void free_quick_find (quick_find * const q) {
  free (q->id);
  for (i32 i = 0; i < q->num; ++i) {
    free (q->elem[i]);
  }
  free (q);
}

i32 get_id (const quick_find * const q, const i32 x) {
  return q->id[x];
}

i32 get_size (const quick_find * const q, const i32 x) {
  return q->size[get_id (q, x)];
}

int same (const quick_find * const q, const i32 x, const i32 y) {
  return get_id (q, x) == get_id (q, y);
}

void unite (quick_find * const q, i32 x, i32 y) {
  x = get_id (q, x);
  y = get_id (q, y);
  if (x == y) {
    return;
  }
  if (q->size[x] < q->size[y]) {
    const i32 s = x;
    x = y;
    y = s;
  }
  if (q->max_size[x] < q->size[x] + q->size[y]) {
    q->max_size[x] *= 2;
    q->elem[x] = realloc (q->elem[x], sizeof (i32) * q->max_size[x]);
  }
  for (i32 i = 0; i < q->size[y]; ++i) {
    const i32 v = q->elem[y][i];
    q->id[v] = x;
    q->elem[x][q->size[x]++] = v;
  }
  free (q->elem[y]);
  q->elem[y] = NULL;
  q->size[y] = 0;
}

const i32 mod = 1000000007;

i32 inv (i32 a) {
  i32 t = 1;
  while (a > 1) {
    t = (i64) t * (mod - mod / a) % mod;
    a = mod % a;
  }
  return t;
}

i32 *fact = NULL;
i32 *iFact = NULL;
void init_fact (const i32 n) {
  fact = (i32 *) calloc (n + 1, sizeof (i32));
  fact[0] = 1;
  for (i32 i = 1; i <= n; ++i) {
    fact[i] = (i64) i * fact[i - 1] % mod;
  }
  iFact = (i32 *) calloc (n + 1, sizeof (i32));
  iFact[n] = inv (fact[n]);
  for (i32 i = n - 1; i >= 0; --i) {
    iFact[i] = (i64) (i + 1) * iFact[i + 1] % mod;
  }
}

#define MIN(a,b) ((a) < (b) ? (a) : (b))

typedef struct node {
  i32 c, w, index;
} node;

int cmp_node_weight (const void *a, const void *b) {
  i32 d = ((node *)a)->w - ((node *)b)->w;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

int cmp_node_color_weight (const void *a, const void *b) {
  i32 d = ((node *)a)->c - ((node *)b)->c;
  if (d != 0) return d < 0 ? -1 : 1;
  return cmp_node_weight (a, b);
}

int cmp_node_index (const void *a, const void *b) {
  i32 d = ((node *)a)->index - ((node *)b)->index;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

void run (void) {
  i32 n, x, y;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &x, &y);
  node *a = (node *) calloc (n, sizeof (node));
  for (i32 i = 0; i < n; ++i) {
    i32 c, w;
    scanf ("%" SCNi32 "%" SCNi32, &c, &w);
    a[i] = (node) {c, w, i};
  }
  quick_find *q = new_quick_find (n);
  qsort (a, n, sizeof (node), cmp_node_color_weight);
  for (i32 i = 0; i < n; ++i) {
    if (i > 0 && a[i].c == a[i - 1].c) continue;
    for (i32 j = i + 1; j < n && a[i].c == a[j].c && a[i].w + a[j].w <= x; ++j) {
      unite (q, a[i].index, a[j].index);
    }
  }
  qsort (a, n, sizeof (node), cmp_node_weight);
  for (i32 i = 1; i < n && a[0].w + a[i].w <= y; ++i) {
    if (a[0].c == a[i].c) continue;
    unite (q, a[0].index, a[i].index);
  }
  for (i32 k = 1; k < n; ++k) {
    if (a[0].c == a[k].c) continue;
    for (i32 j = 0; j < n && a[k].w + a[j].w <= y; ++j) {
      if (a[k].c == a[j].c) continue;
      unite (q, a[k].index, a[j].index);
    }
    k = n;
  }
  qsort (a, n, sizeof (node), cmp_node_index);
  init_fact (n);
  i32 ans = 1;
  i32 *cnt = (i32 *) calloc (n + 1, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    if (q->size[i] <= 1) continue;
    ans = (i64) ans * fact[q->size[i]] % mod;
    for (i32 j = 0; j < q->size[i]; ++j) {
      i32 k = q->elem[i][j];
      cnt[a[k].c]++;
    }
    for (i32 j = 0; j < q->size[i]; ++j) {
      i32 k = q->elem[i][j];
      ans = (i64) ans * iFact[cnt[a[k].c]] % mod;
      cnt[a[k].c] = 0;
    }
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}
