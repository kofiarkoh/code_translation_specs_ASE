#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<assert.h>

typedef int32_t i32;
typedef int64_t i64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))
#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))
#define SORT(a,num,cmp) qsort((a),(num),sizeof(*(a)),cmp)

typedef struct graph_edge {
  i32 v, c, d, next;
} graph_edge;

typedef struct graph {
  graph_edge *edge;
  i32 *start;
  i32 size;
  i32 p;
} graph;

graph* new_graph (i32 v, i32 e) {
  graph *g = ALLOC (1, graph);
  g->edge = ALLOC (e, graph_edge);
  g->start = ALLOC (v, i32);
  g->size = v;
  g->p = 0;
  for (i32 i = 0; i < v; ++i) {
    g->start[i] = -1;
  }
  return g;
}

void add_edge (graph *g, i32 a, i32 b, i32 c, i32 d) {
  g->edge[g->p] = (graph_edge) {b, c, d, g->start[a]};
  g->start[a] = g->p++;
}

typedef struct LCA_node {
  i32 vertex;
  i32 depth;
} LCA_node;

LCA_node min_LCA_node (LCA_node a, LCA_node b) {
  return a.depth <= b.depth ? a : b;
}

typedef struct lowest_common_ancestor_structure {
  i32 *left;
  i32 *right;
  LCA_node *s;
  i32 size;
} LCA;

void fill_lca (i32 v, i32 d, graph *g, LCA *lca, i32 *index, uint8_t *used) {
  used[v] = 1;
  lca->left[v] = *index;
  lca->right[v] = *index;
  lca->s[*index + lca->size] = (LCA_node) {v, d};
  *index += 1;
  for (i32 p = g->start[v]; p != -1; p = g->edge[p].next) {
    i32 u = g->edge[p].v;
    if (used[u]) continue;
    fill_lca (u, d + 1, g, lca, index, used);
    lca->right[v] = *index;
    lca->s[*index + lca->size] = (LCA_node) {v, d};
    *index += 1;
  }
}

LCA* build_LCA (graph *g) {
  const i32 n = g->size;
  i32 size = 1;
  while (size < 2 * n - 1) size *= 2;
  LCA *lca = ALLOC (1, LCA);
  lca->left = ALLOC (n, i32);
  lca->right = ALLOC (n, i32);
  lca->s = ALLOC (2 * size, LCA_node);
  lca->size = size;
  uint8_t *used = ALLOC (n, uint8_t);
  i32 index = 0;
  fill_lca (0, 0, g, lca, &index, used);
  for (i32 i = size - 1; i >= 1; --i) {
    lca->s[i] = min_LCA_node (lca->s[2 * i], lca->s[2 * i + 1]);
  }
  free (used);
  return lca;
}

i32 query (LCA *lca, i32 v, i32 u) {
  i32 l = MIN (lca->left[v], lca->left[u]);
  i32 r = MAX (lca->right[v], lca->right[u]) + 1;
  LCA_node ans = {-1, lca->size + 1};
  for (l += lca->size, r += lca->size; l < r; l >>= 1, r >>= 1) {
    if (l & 1) ans = min_LCA_node (ans, lca->s[l++]);
    if (r & 1) ans = min_LCA_node (ans, lca->s[--r]);
  }
  return ans.vertex;
}

typedef struct node {
  i32 v;
  i32 x;
  i32 y;
  i32 d;
} node;

int cmp_node (const void *a, const void *b) {
  const node *p = a;
  const node *q = b;
  i32 d = p->v - q->v;
  if (d != 0) return d < 0 ? -1 : 1;
  d = p->x - q->x;
  if (d != 0) return d < 0 ? -1 : 1;
  d = p->y - q->y;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

i32 to_index (node *z, i32 len, node *v) {
  i32 l = 0;
  i32 r = len;
  while (r - l > 1) {
    i32 m = (l + r) / 2;
    if (cmp_node (z + m, v) <= 0) {
      l = m;
    } else {
      r = m;
    }
  }
  return l;
}

void euler_tour (i32 v, i32 d, i32 *sum, i32 *cnt, graph *g, node *z, i32 len, uint8_t *used) {
  used[v] = 1;
  i32 s = to_index (z, len, &(node){v, 0, 0, 0}) + 1;
  for (i32 i = s; i < len && z[i].v == v; ++i) {
    i32 x = z[i].x;
    z[i].d = d - sum[x] + cnt[x] * z[i].y;
  }
  for (i32 p = g->start[v]; p != -1; p = g->edge[p].next) {
    i32 u = g->edge[p].v;
    if (used[u]) continue;
    i32 c = g->edge[p].c;
    i32 add = g->edge[p].d;
    sum[c] += add;
    cnt[c]++;
    euler_tour (u, d + add, sum, cnt, g, z, len, used);
    sum[c] -= add;
    cnt[c]--;
  }
}

void run (void) {
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  graph *g = new_graph (n, 2 * n);
  for (i32 i = 0; i < n - 1; ++i) {
    i32 a, b, c, d;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32 "%" SCNi32, &a, &b, &c, &d);
    a--; b--;
    add_edge (g, a, b, c, d);
    add_edge (g, b, a, c, d);
  }
  LCA *lca = build_LCA (g);
  i32 *x = ALLOC (q, i32);
  i32 *y = ALLOC (q, i32);
  i32 *u = ALLOC (q, i32);
  i32 *v = ALLOC (q, i32);
  node *z = ALLOC (3 * q, node);
  for (i32 i = 0; i < q; ++i) {
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32 "%" SCNi32, x + i, y + i, u + i, v + i);
    u[i]--; v[i]--;
    assert (0 < x[i] && x[i] < n);
    z[3 * i] = (node) {u[i], x[i], y[i], 0};
    z[3 * i + 1] = (node) {v[i], x[i], y[i], 0};
    z[3 * i + 2] = (node) {query (lca, u[i], v[i]), x[i], y[i], 0};
  }
  SORT (z, 3 * q, cmp_node);
  i32 len = 1;
  for (i32 i = 1; i < 3 * q; ++i) {
    if (cmp_node (z + i, z + len - 1) == 0) continue;
    z[len++] = z[i];
  }
  i32 *sum = ALLOC (n, i32);
  i32 *cnt = ALLOC (n, i32);
  uint8_t *used = ALLOC (n, uint8_t);
  euler_tour (0, 0, sum, cnt, g, z, len, used);
  for (i32 i = 0; i < q; ++i) {
    i32 ans = 0;
    ans += z[to_index (z, len, &(node){u[i], x[i], y[i], 0})].d;
    ans += z[to_index (z, len, &(node){v[i], x[i], y[i], 0})].d;
    ans -= 2 * z[to_index (z, len, &(node){query (lca, u[i], v[i]), x[i], y[i], 0})].d;
    printf ("%" PRIi32 "\n", ans);
  }
}

int main (void) {
  run();
  return 0;
}
