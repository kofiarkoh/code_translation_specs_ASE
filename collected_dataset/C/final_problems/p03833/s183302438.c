#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct segment_tree_void {
  void *array;
  int32_t size;
  size_t val_size;
  void (*func) (void *, const void *, const void *);
} segment_tree;

static inline void segment_tree_memcpy (void *dst, const void *src, size_t size) {
  uint8_t *p = dst;
  const uint8_t *q = src;
  while (size--) {
    *p++ = *q++;
  }
}

segment_tree* new_segment_tree (const void *ini, const int32_t n, const size_t val_size, void (*func) (void *, const void *, const void *)) {
  int32_t k = 1;
  while (k < n) k *= 2;
  segment_tree *s = (segment_tree *) calloc (1, sizeof (segment_tree));
  s->array = calloc (2 * k, val_size);
  s->size = k;
  s->val_size = val_size;
  s->func = func;
  for (int32_t i = k; i < 2 * k; ++i) {
    segment_tree_memcpy ((uint8_t *)s->array + i * val_size, ini, val_size);
  }
  uint8_t *a = s->array;
  for (int32_t i = k - 1; i > 0; --i) {
    func (a + i * val_size, a + (2 * i) * val_size, a + (2 * i + 1) * val_size);
  }
  return s;
}

void update (segment_tree * const s, int32_t k, const void *val) {
  k += s->size;
  uint8_t * const p = s->array;
  const size_t size = s->val_size;
  segment_tree_memcpy (p + k * size, val, size);
  for (k >>= 1; k > 0; k >>= 1) {
    s->func (p + k * size, p + 2 * k * size, p + (2 * k + 1) * size);
  }
}

void update_tmp (segment_tree * const s, int32_t k, const void *val) {
  k += s->size;
  uint8_t * const p = s->array;
  const size_t size = s->val_size;
  segment_tree_memcpy (p + k * size, val, size);
}

void update_all (segment_tree * const s) {
  uint8_t * const a = s->array;
  const size_t size = s->val_size;
  for (int32_t i = s->size - 1; i > 0; --i) {
    s->func (a + i * size, a + (2 * i) * size, a + (2 * i + 1) * size);
  }
}

void find (const segment_tree *s, int32_t l, int32_t r, void *res) {
  int32_t lindex[64];
  int32_t rindex[32];
  int32_t llen = 0;
  int32_t rlen = 0;
  for (l += s->size, r += s->size; l < r; l >>= 1, r >>= 1) {
    if (l & 1) lindex[llen++] = l++;
    if (r & 1) rindex[rlen++] = --r;
  }
  while (rlen > 0) {
    lindex[llen++] = rindex[--rlen];
  }
  const uint8_t *p = s->array;
  segment_tree_memcpy (res, p + lindex[0] * s->val_size, s->val_size);
  for (int32_t i = 1; i < llen; ++i) {
    s->func (res, res, p + lindex[i] * s->val_size);
  }
}

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))
#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))
#define SORT(a,num,cmp) qsort((a),(num),sizeof(*(a)),cmp)

#define POS(i, j) ((i) * (n + 2) + (j))

typedef struct index_val {
  i32 v;
  i32 index;
} node;

void func_max (void *c, const void *a, const  void *b) {
  const node *p = a;
  const node *q = b;
  *(node *)c = p->v > q->v ? *p : *q;
}

typedef struct range {
  i32 l, r;
} range;

void run (void) {
  i32 n, m;
  scanf ("%" SCNi32 "%" SCNi32, &n, &m);
  i64 *a = ALLOC (n, i64);
  for (i32 i = 0; i < n - 1; ++i) {
    scanf ("%" SCNi64, a + i);
  }
  for (i32 i = n - 2; i >= 0; --i) {
    a[i] += a[i + 1];
  }
  i32 *b = ALLOC (n * m, i32);
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = 0; j < m; ++j) {
      scanf ("%" SCNi32, b + i * m + j);
    }
  }
  i64 *dp = ALLOC ((n + 2) * (n + 2), i64);
  const node ini = {-1, 0};
  segment_tree *s = new_segment_tree (&ini, n, sizeof (node), func_max);
  range *stack = ALLOC (2 * n, range);
  for (i32 i = 0; i < m; ++i) {
    for (i32 j = 0; j < n; ++j) {
      node t = {b[j * m + i], j};
      update_tmp (s, j, &t);
    }
    update_all (s);
    i32 top = 0;
    stack[top++] = (range) {0, n};
//    printf ("Ticket %" PRIi32 "\n", i);
    while (top > 0) {
      range op = stack[--top];
      if (op.l >= op.r) continue;
      node t;
      find (s, op.l, op.r, &t);
//      printf ("[%" PRIi32 ", %" PRIi32 ") = (%" PRIi32 ", %" PRIi32 ")\n", op.l, op.r, t.v, t.index);
      dp[POS(op.l       , t.index + 1)]  += t.v;
      dp[POS(t.index + 1, t.index + 1)]  -= t.v;
      dp[POS(op.l       , op.r + 1)]     -= t.v;
      dp[POS(t.index + 1, op.r + 1)]     += t.v;
      stack[top++] = (range) {op.l, t.index};
      stack[top++] = (range) {t.index + 1, op.r};
    }
  }
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = 1; j <= n; ++j) {
      dp[POS(i, j)] += dp[POS(i, j - 1)];
    }
  }
  for (i32 j = 0; j <= n; ++j) {
    for (i32 i = 1; i < n; ++i) {
      dp[POS(i, j)] += dp[POS(i - 1, j)];
    }
  }
  i64 max = 0;
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = i + 1; j <= n; ++j) {
      i64 v = dp[POS(i, j)] - (a[i] - a[j - 1]);
//      printf ("[%" PRIi32 ", %" PRIi32 ") = %" PRIi64 "\n", i, j, dp[POS(i,j)]);
      if (max < v) {
        max = v;
      }
    }
  }
  printf ("%" PRIi64 "\n", max);
}

int main (void) {
  run();
  return 0;
}
