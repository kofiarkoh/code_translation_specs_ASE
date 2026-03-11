#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct directed_edge {
  int32_t vertex;
  int32_t next;
} graph_edge;

typedef struct directedGraph {
  graph_edge *edge;
  int32_t *start;
  int32_t pointer;
  int32_t vertex_num;
  int32_t edge_max_size;
} graph;

graph* new_graph (const int vertex_num) {
  graph *g = (graph *) calloc (1, sizeof (graph));
  g->edge = (graph_edge *) calloc (1, sizeof (graph_edge));
  g->start = (int32_t *) calloc (vertex_num, sizeof (int32_t));
  g->pointer = 0;
  g->vertex_num = vertex_num;
  g->edge_max_size = 1;
  for (int32_t i = 0; i < vertex_num; ++i) {
    g->start[i] = -1;
  }
  return g;
}

void add_edge (graph *g, int32_t from, int32_t to) {
  if (g->pointer == g->edge_max_size) {
    g->edge_max_size *= 2;
    g->edge = (graph_edge *) realloc (g->edge, sizeof (graph_edge) * g->edge_max_size);
  }
  g->edge[g->pointer] = (graph_edge) {to, g->start[from]};
  g->start[from] = g->pointer++;
}

i32* BFS_graph (graph *g, int32_t src) {
  int32_t *q = (int32_t *) calloc (g->vertex_num, sizeof (int32_t));
  uint8_t *used = (uint8_t *) calloc (g->vertex_num, sizeof (uint8_t));
  int32_t front = 0;
  int32_t last = 0;
  used[src] = 1;
  q[last++] = src;
  while (front < last) {
    const int32_t v = q[front++];
    for (int32_t p = g->start[v]; p != -1; p = g->edge[p].next) {
      const int32_t u = g->edge[p].vertex;
      if (!used[u]) {
	used[u] = 1;
	q[last++] = u;
      }
    }
  }
  free(used);
  return q;
}

const i32 mod = 1000000007;

i32 mod_pow (i32 r, i32 n) {
  i32 t = 1;
  i32 s = r;
  while (n > 0) {
    if (n & 1) t = (i64) t * s % mod;
    s = (i64) s * s % mod;
    n >>= 1;
  }
  return t;
}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  if (n >= 2000) exit (1);
  i32 *p = (i32 *) calloc (n + 1, sizeof (i32));
  i32 *depth = (i32 *) calloc (n + 1, sizeof (i32));
  graph *g = new_graph (n + 1);
  for (i32 i = 1; i <= n; ++i) {
    scanf ("%" SCNi32, p + i);
    add_edge (g, p[i], i);
    depth[i] = depth[p[i]] + 1;
  }
  i32 *q = BFS_graph (g, 0);
  i32 *iq = (i32 *) calloc (n + 1, sizeof (i32));
  for (i32 i = 0; i <= n; ++i) iq[q[i]] = i;
  i32 *zero = (i32 *) calloc (n + 1, sizeof (i32));
  i32 *one  = (i32 *) calloc (n + 1, sizeof (i32));
  i64 ans = 0;
  for (i32 i = 0, d = 0; i <= n; ++d) {
    i32 s = i;
    for (; i <= n && depth[q[i]] == d; ++i);
    for (i32 j = s; j < i; ++j) {
      one[j] = 1;
      zero[j] = 1;
    }
    for (i32 j = s - 1; j >= 0; --j) {
      i64 z[2] = {1, 0};
      i64 total = 1;
      for (i32 p = g->start[q[j]]; p != -1; p = g->edge[p].next) {
	i32 u = iq[g->edge[p].vertex];
	i64 a = z[0] * one[u] + z[1] * zero[u];
	i64 b = z[0] * zero[u];
	total = total * (one[u] + zero[u]) % mod;
	z[1] = a % mod;
	z[0] = b % mod;
      }
      one[j] = z[1];
      zero[j] = (total + mod - z[1]) % mod;
    }
    ans += (i64)mod_pow (2, n + 1 - (i - s)) * one[0] % mod;
  }
  printf ("%" PRIi64 "\n", ans % mod);
}

int main (void) {
  run();
  return 0;
}
