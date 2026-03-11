#pragma GCC optimize("Ofast")
#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef struct UnionFind {
  int32_t *parent;
  int32_t size;
} UnionFind;

void initUnionFind (UnionFind *u) {
  for (int32_t i = 0; i < u->size; ++i){
    u->parent[i] = -1;
  }
}

UnionFind* newUnionFind (const int32_t size) {
  UnionFind *u = (UnionFind *) calloc (1, sizeof (UnionFind));
  u->parent = (int32_t *) calloc (size, sizeof (int32_t));
  u->size = size;
  initUnionFind (u);
  return u;
}

void freeUnionFind (UnionFind *u) {
  free (u->parent);
  free (u);
}

int32_t root (UnionFind *u, int32_t x) {
  if (u->parent[x] < 0) return x;
  return u->parent[x] = root (u, u->parent[x]);
}

int same (UnionFind *u, int32_t x, int32_t y) {
  return root (u, x) == root (u, y);
}

int32_t getSize(UnionFind *u, int32_t x) {
  return - (u->parent[root (u, x)]);
}

void unite(UnionFind *u, int32_t x, int32_t y) {
  x = root (u, x);
  y = root (u, y);
  if (x == y) return;
  if (u->parent[x] > u->parent[y]) {
    int32_t swap = x;
    x = y;
    y = swap;
  }
  u->parent[x] += u->parent[y];
  u->parent[y] = x;
}

typedef int32_t i32;
typedef int64_t i64;

typedef struct edge {
  i32 a, b;
  i64 c;
} edge;

typedef edge heap_val;
typedef int32_t heap_index;

const heap_index null_index = -1;

static inline int pairingheap_func_compare (heap_val a, heap_val b) {
  i64 d = a.c - b.c;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

typedef struct pairingHeapNode {
  heap_index next;
  heap_index child;
  heap_val val;
} heap_node;

typedef struct pairingHeapHead {
  heap_node *array;
  heap_index root;
  heap_index free_front;
  int32_t heap_size;
  int32_t array_len;
} heap;

void init_pairingheap (heap * const h) {
  if (h->array != NULL) {
    free (h->array);
  }
  const int32_t initial_len = 1;
  h->array = (heap_node *) calloc (initial_len, sizeof (heap_node));
  h->array[0].next = null_index;
  h->root = null_index;
  h->free_front = 0;
  h->array_len = initial_len;
  h->heap_size = 0;
}

heap* new_pairingheap (void) {
  heap *h = (heap *) calloc (1, sizeof (heap));
  h->array = NULL;
  init_pairingheap (h);
  return h;
}

int32_t get_heap_size (const heap *h) {
  return h->heap_size;
}

int is_empty (const heap *h) {
  return h->heap_size == 0;
}

static inline heap_index meld (heap_node * const array, heap_index a, heap_index b) {
  if (a == null_index) return b;
  if (b == null_index) return a;
  heap_node *p = array + a;
  heap_node *q = array + b;
  if (pairingheap_func_compare (p->val, q->val) > 0) {
    const heap_index swap_index = a;
    a = b;
    b = swap_index;
    heap_node * const swap_ptr = p;
    p = q;
    q = swap_ptr;
  } 
  q->next = p->child;
  p->child = b;
  return a;
}

void push (heap * const h, const heap_val val) {
  if (h->free_front == null_index) {
    const heap_index front = h->array_len;
    h->array_len *= 2;
    h->array = (heap_node *) realloc (h->array, sizeof (heap_node) * h->array_len);
    h->free_front = front;
    for (heap_index i = front; i + 1 < h->array_len; ++i) {
      h->array[i].next = i + 1;
    }
    h->array[h->array_len - 1].next = null_index;
  }
  h->heap_size++;
  const heap_index k = h->free_front;
  h->free_front = h->array[h->free_front].next;
  h->array[k] = (heap_node) {null_index, null_index, val};
  h->root = meld (h->array, h->root, k);
}

heap_val pop (heap *h) {
  h->heap_size--;
  heap_node * const array = h->array;
  const heap_val res = array[h->root].val;
  array[h->root].next = h->free_front;
  h->free_front = h->root;
  heap_index lst = array[h->root].child;
  heap_index r = null_index;
  while (lst != null_index && array[lst].next != null_index) {
    const heap_index a = lst;
    const heap_index b = array[a].next;
    lst = array[b].next;
    array[a].next = array[b].next = null_index;
    r = meld (array, r, meld (array, a, b));
  }
  if (lst != null_index) {
    r = meld (array, r, lst);
  }
  h->root = r;
  return res;
}

void build (const i32 *a, const i32 n, const i32 d, heap * const h) {
  if (n <= 1) return;
  static i32 buf = 0;
  i32 m = n / 2;
  i32 min = 0;
  for (i32 i = 1; i < m; ++i) {
    i64 x = (i64) d * (i - min) + a[min];
    i64 y = a[i];
    if (x > y) {
      min = i;
    }
  }
  for (i32 i = m; i < n; ++i) {
    push (h,(edge){buf + min, buf + i, (i64) d * (i - min) + a[min] + a[i]});
  }
  min = n - 1;
  for (i32 i = n - 2; i >= m; --i) {
    i64 x = (i64) d * (min - i) + a[min];
    i64 y = a[i];
    if (x > y) {
      min = i;
    }
  }
  for (i32 i = m - 1; i >= 0; --i) {
    push (h, (edge){buf + i, buf + min, (i64) d * (min - i) + a[min] + a[i]});
  }
  build (a, m, d, h);
  buf += m;
  build (a + m, n - m, d, h);
  buf -= m;
}

void run (void) {
  i32 n, d;
  scanf ("%" SCNi32 "%" SCNi32, &n, &d);
  i32 *a = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  heap *h = new_pairingheap ();
  build (a, n, d, h);
  UnionFind *u = newUnionFind (n);
  i64 sum = 0;
  while (getSize (u, 0) < n) {
    edge e = pop(h);
    if (same (u, e.a, e.b)) continue;
    sum += e.c;
    unite (u, e.a, e.b);
  }
  printf ("%" PRIi64 "\n", sum);
}

int main (void) {
  run ();
  return 0;
}
