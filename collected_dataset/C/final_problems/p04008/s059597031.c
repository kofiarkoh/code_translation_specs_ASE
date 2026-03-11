#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

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

void BFS_graph (graph *g, int32_t src, i32 *q, i32 *d) {
  uint8_t *used = (uint8_t *) calloc (g->vertex_num, sizeof (uint8_t));
  int32_t front = 0;
  int32_t last = 0;
  used[src] = 1;
  q[last++] = src;
  d[src] = 0;
  while (front < last) {
    const int32_t v = q[front++];
    for (int32_t p = g->start[v]; p != -1; p = g->edge[p].next) {
      const int32_t u = g->edge[p].vertex;
      if (!used[u]) {
	used[u] = 1;
	q[last++] = u;
	d[u] = d[v] + 1;
      }
    }
  }
  free(used);
}

void fill (graph *g, i32 v, i32 *d, i32 *ok) {
  if (ok[v]) return;
  ok[v] = 1;
  for (i32 p = g->start[v]; p != -1; p = g->edge[p].next) {
    i32 u = g->edge[p].vertex;
    if (d[u] > d[v]) {
      fill (g, u, d, ok);
    }
  }
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 *a = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
    a[i]--;
  }
  graph *g = new_graph (n);
  for (i32 i = 1; i < n; ++i) {
    add_edge (g, i, a[i]);
    add_edge (g, a[i], i);
  }
  i32 *q = (i32 *) calloc (n, sizeof (i32));
  i32 *d = (i32 *) calloc (n, sizeof (i32));
  BFS_graph (g, 0, q, d);
  i32 *ok = (i32 *) calloc (n, sizeof (i32));
  i32 ans = a[0] == 0 ? 0 : 1;
  a[0] = 0;
  ok[0] = 1;
  for (i32 i = n - 1; i > 0; --i) {
    i32 v = q[i];
    if (ok[v]) continue;
    i32 t = k - 1;
    while (t > 0 && v > 0 && !ok[v]) {
      for (i32 p = g->start[v]; p != -1; p = g->edge[p].next) {
        i32 u = g->edge[p].vertex;
        if (d[u] < d[v]) {
          v = u;
        }
      }
      t--;
    }
    if (v > 0 && a[v] != 0 && !ok[v]) {
      ans++;
    }
    fill (g, v, d, ok);
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}
