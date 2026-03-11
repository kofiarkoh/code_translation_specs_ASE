#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#define MIN(a,b) ((a) < (b) ? (a) : (b))
struct node {
  int id;
  int cap;
  struct node *next;
};
struct node **list;
int **flow, *iter, *dist, *prev, *queue;
void Insert(int, int, int);
void Init_bfs(int, int);
int Maxflow_dfs(int, int, int, int);
int main(void)
{
  int i, v, e, s, t, c, maxflow = 0;
  scanf("%d %d", &v, &e);
  list = (struct node**)malloc(sizeof(struct node*) * v);
  flow = (int**)malloc(sizeof(int*) * v);
  iter = (int*)malloc(sizeof(int) * v);
  dist = (int*)malloc(sizeof(int) * v);
  prev = (int*)malloc(sizeof(int) * v);
  queue = (int*)malloc(sizeof(int) * v);
  for (i = 0; i < v; i++) {
    list[i] = NULL;
    flow[i] = (int*)calloc(v, sizeof(int));
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d %d", &s, &t, &c);
    Insert(s, t, c);
  }
  while (1) {
    Init_bfs(0, v);
    if (dist[v-1] < 0) break;
    for (i = 0; i < v; i++) iter[i] = 0;
    while (1) {
      int f = Maxflow_dfs(0, v - 1, v, INT_MAX);
      if (!f) break;
      maxflow += f;
    }
  }
  printf("%d\n", maxflow);
  for (i = 0; i < v; i++) {
    free(list[i]);
    free(flow[i]);
  }
  free(list);
  free(flow);
  free(iter);
  free(dist);
  free(prev);
  free(queue);
}
void Insert(int a, int b, int c)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->cap = c;
  p->next = list[a];
  list[a] = p;
  p = (struct node*)malloc(sizeof(struct node));
  p->id = a;
  p->cap = 0;
  p->next = list[b];
  list[b] = p;
}
void Init_bfs(int s, int size)
{
  struct node *n;
  int i, head = 0, tail = 0;
  for (i = 0; i < size; i++) dist[i] = -1;
  dist[s] = 0;
  queue[tail++] = s;
  while (head != tail) {
    i = queue[head++];
    for (n = list[i]; n != NULL; n = n->next) {
      int v = n->id;
      if (dist[v] < 0 && flow[i][v] < n->cap) {
        dist[v] = dist[i] + 1;
        queue[tail++] = v;
      }
    }
  }
}
int Maxflow_dfs(int s, int t, int size, int f)
{
  struct node *n = list[s];
  if (s == t) return f;
  while (n != NULL && iter[s]) {
    n = n->next;
    iter[s]--;
  }
  for (; n != NULL; n = n->next) {
    int v = n->id;
    iter[s] = v;
    if (dist[s] < dist[v] && flow[s][v] < n->cap) {
      int d = Maxflow_dfs(v, size - 1, size, MIN(f, n->cap - flow[s][v]));
      if (d > 0) {
        flow[s][v] += d;
        flow[v][s] -= d;
        return d;
      }
    }
  }
  return 0;
}
