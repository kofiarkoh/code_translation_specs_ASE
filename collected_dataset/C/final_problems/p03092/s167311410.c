#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int64_t flow_type;

const flow_type flow_inf = (int64_t) 1000000000 * 5000;

typedef struct flow_edge {
  int32_t vertex;
  int32_t rev_index;
  flow_type capacity;
} flow_edge;

typedef struct flow_edge_vector {
  flow_edge *edge;
  int32_t length;
  int32_t max_length;
} flow_edge_vector;

typedef struct maxFlowGraph {
  flow_edge_vector *from;
  int32_t vertex_num;
} graph;

graph* new_graph (const int32_t vertex_num) {
  graph *g = (graph *) calloc (1, sizeof (graph));
  g->vertex_num = vertex_num;
  g->from = (flow_edge_vector *) calloc (vertex_num, sizeof (flow_edge_vector)); 
  for (int32_t i = 0; i < vertex_num; ++i) {
    g->from[i].edge = (flow_edge *) calloc (vertex_num / 2, sizeof (flow_edge));
    g->from[i].length = 0;
    g->from[i].max_length = vertex_num / 2;
  }
  return g;
}

void add_edge_sub (flow_edge_vector *v, int32_t vertex, int32_t rev_index, int64_t capa) {
  if (v->length == v->max_length) {
    v->max_length *= 2;
    v->edge = (flow_edge *) realloc (v->edge, sizeof (flow_edge) * v->max_length);
  }
  v->edge[v->length++] = (flow_edge) {vertex, rev_index, capa};
}

void add_edge (graph * const g, const int32_t src, const int32_t dst, const flow_type capa) {
  add_edge_sub (&g->from[src], dst, g->from[dst].length, capa);
  add_edge_sub (&g->from[dst], src, g->from[src].length - 1, 0);
}

flow_type dinic_dfs (const int32_t v, graph * const g, const int32_t src, const int32_t * const level, int32_t * const iter, flow_type e) {
  if (v == src) return e;
  flow_type sum = 0;
  flow_edge_vector * const vec = &g->from[v];
  for (int32_t p = iter[v]; p < vec->length; ++p, iter[v] = p) {
    const int32_t u = vec->edge[p].vertex;
    const flow_type capa = g->from[u].edge[vec->edge[p].rev_index].capacity;
    if (level[u] >= level[v] || capa <= 0) continue;
    const flow_type f = dinic_dfs (u, g, src, level, iter, capa < e ? capa : e);
    if (f > 0) {
      vec->edge[p].capacity += f;
      g->from[u].edge[vec->edge[p].rev_index].capacity -= f;
      sum += f;
      e -= f;
      if (e <= 0) return sum;
    }
  }
  return sum;
}

flow_type dinic (graph * const g, const int32_t src, const int32_t dst) {
  const int32_t vertex_num = g->vertex_num;
  int32_t * const level = (int32_t *) calloc (vertex_num, sizeof (int32_t));
  int32_t * const queue = (int32_t *) calloc (vertex_num, sizeof (int32_t));
  int32_t * const iter  = (int32_t *) calloc (vertex_num, sizeof (int32_t));
  flow_type flow = 0;
  while (1) {
    memset (level, 0, sizeof (int32_t) * vertex_num);
    int32_t front = 0;
    int32_t last = 0;
    level[src] = 1;
    queue[last++] = src;
    while (front < last && level[dst] == 0) {
      const int32_t v = queue[front++];
      const flow_edge_vector *vec = &g->from[v];
      for (int32_t p = 0; p < vec->length; ++p) {
        const int32_t u = vec->edge[p].vertex;
        if (vec->edge[p].capacity > 0 && level[u] == 0) {
          level[u] = level[v] + 1;
          queue[last++] = u;
        }
      }
    }
    if (level[dst] == 0) break;
    memset (iter, 0, sizeof (int32_t) * vertex_num);
    while (1) {
      flow_type f = dinic_dfs (dst, g, src, level, iter, flow_inf);
      if (f <= 0) break;
      flow += f;
    }
  }
  free (level);
  free (queue);
  free (iter);
  return flow;
}

typedef int32_t i32;
typedef int64_t i64;

void run (void) {
  i32 n, a, b;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &a, &b);
  i32 *p = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, p + i);
    p[i]--;
  }
  graph *g = new_graph (2 * n + 2);
  const i32 src = 2 * n;
  const i32 dst = 2 * n + 1;
  for (i32 i = 0; i < n; ++i) {
    add_edge (g, i + n, dst, b);
    add_edge (g, src, i, a);
  }
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = i + 1; j < n; ++j) {
      if (p[i] > p[j]) {
        add_edge (g, i, j + n, flow_inf);
      }
    }
  }
  i64 ans = dinic (g, src, dst);
  printf("%" PRIi64 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}
