#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#define MIN(a,b) ((a) < (b) ? (a) : (b))
struct node {
  int id;
  int cap;
  int cost;
  struct node *next;
};
struct node **list;
int **flow, *delta, *dist,*prev;
int *heap, *heap_index, heapsize;
void Insert(int, int, int, int);
void downheap(int);
void upheap(int);
void PQ_init(int);
int PQ_remove(void);
void PQ_update(int);
int Maxflow(int, int, int);
int main(void)
{
  int i, v, e, f, s, t, c, d, mincost = 0;
  scanf("%d %d %d", &v, &e, &f);
  list = (struct node**)malloc(sizeof(struct node*) * v);
  flow = (int**)malloc(sizeof(int*) * v);
  delta = (int*)malloc(sizeof(int) * v);
  dist = (int*)malloc(sizeof(int) * v);
  prev = (int*)malloc(sizeof(int) * v);
  for (i = 0; i < v; i++) {
    list[i] = NULL;
    flow[i] = (int*)calloc(v, sizeof(int));
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d %d %d", &s, &t, &c, &d);
    Insert(s, t, c, d);
  }
  while (Maxflow(0, v - 1, v) && f) {
    int n = v - 1;
    do {
      flow[prev[n]][n] += delta[v-1];
      flow[n][prev[n]] -= delta[v-1];
      n = prev[n];
    } while(n);
    f -= delta[v-1];
  }
  if (!f) {
    for (i = 0; i < v; i++) {
      struct node *n;
      for (n = list[i]; n != NULL; n = n->next) {
        mincost += flow[i][n->id] * n->cost;
      }
    }
    printf("%d\n", mincost);
  } else printf("-1\n");
  for (i = 0; i < v; i++) {
    free(list[i]);
    free(flow[i]);
  }
  free(list);
  free(flow);
  free(delta);
  free(dist);
  free(prev);
}
void Insert(int a, int b, int cap, int cost)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->cap = cap;
  p->cost = cost;
  p->next = list[a];
  list[a] = p;
  p = (struct node*)malloc(sizeof(struct node));
  p->id = a;
  p->cap = 0;
  p->cost = 0;
  p->next = list[b];
  list[b] = p;
}
void downheap(int k)
{
  int j, v = heap[k];
  while (k < heapsize / 2) {
    j = 2 * k + 1;
    if (j < heapsize - 1 && dist[heap[j]] > dist[heap[j+1]]) j++;
    if (dist[v] <= dist[heap[j]]) break;
    heap[k] = heap[j];
    heap_index[heap[j]] = k;
    k = j;
  }
  heap[k] = v;
  heap_index[v] = k;
}
void upheap(int j)
{
  int k, v = heap[j];
  while (j > 0) {
    k = (j + 1) / 2 -1;
    if (dist[v] >= dist[heap[k]]) break;
    heap[j] = heap[k];
    heap_index[heap[k]] = j;
    j = k;
  }
  heap[j] = v;
  heap_index[v] = j;
}
void PQ_init(int size)
{
  int i;
  heapsize = size;
  heap = (int*)malloc(sizeof(int) * size);
  heap_index = (int*)malloc(sizeof(int) * size);
  for (i = 0; i < size; i++) {
    heap[i] = i;
    heap_index[i] = i;
  }
  for (i = heapsize / 2 - 1; i >= 0; i--) downheap(i);
}
int PQ_remove(void)
{
  int v = heap[0];
  heap[0] = heap[heapsize-1];
  heap_index[heap[heapsize-1]] = 0;
  heapsize--;
  downheap(0);
  return v;
}
void PQ_update(int v)
{
  upheap(heap_index[v]);
}
int Maxflow(int s, int t, int size)
{
  struct node *n;
  int i;
  for (i = 0; i < size; i++) {
    delta[i] = INT_MAX;
    dist[i] = INT_MAX;
    prev[i] = -1;
  }
  dist[s] = 0;
  PQ_init(size);
  while(heapsize) {
    i = PQ_remove();
    if (i == t) break;
    for (n = list[i]; n != NULL; n = n->next) {
      int v = n->id;
      if (flow[i][v] < n->cap) {
        int newlen = dist[i] + n->cost;
        if (newlen < dist[v]) {
          dist[v] = newlen;
          prev[v] = i;
          delta[v] = MIN(delta[i], n->cap - flow[i][v]);
          PQ_update(v);
        }
      }
    }
  }
  free(heap);
  free(heap_index);
  return dist[t] != INT_MAX;
}
