#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

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

void BFS_graph (graph *g, int32_t src, i32 *q, i32 *parent) {
  uint8_t *used = (uint8_t *) calloc (g->vertex_num, sizeof (uint8_t));
  int32_t front = 0;
  int32_t last = 0;
  used[src] = 1;
  q[last++] = src;
  parent[src] = -1;
  while (front < last) {
    const int32_t v = q[front++];
    for (int32_t p = g->start[v]; p != -1; p = g->edge[p].next) {
      const int32_t u = g->edge[p].vertex;
      //hoge
      if (!used[u]) {
        used[u] = 1;
        q[last++] = u;
        parent[u] = v;
      }
    }
  }
  free(used);
}

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  graph *g = new_graph (n);
  for (i32 i = 1; i < n; ++i) {
    i32 a, b;
    scanf ("%" SCNi32 "%" SCNi32, &a, &b);
    a--; b--;
    add_edge (g, a, b);
    add_edge (g, b, a);
  }
  i32 *q = ALLOC (n, i32);
  i32 *parent = ALLOC (n, i32);
  BFS_graph (g, 0, q, parent);
  i32 *dp = ALLOC (n, i32);
  for (i32 i = n - 1; i >= 0; --i) {
    i32 v = q[i];
    for (i32 p = g->start[v]; p != -1; p = g->edge[p].next) {
      i32 u = g->edge[p].vertex;
      if (u == parent[v]) continue;
      dp[v] ^= 1 + dp[u];
    }
  }
  puts (dp[0] > 0 ? "Alice" : "Bob");
}

int main (void) {
  run();
  return 0;
}
