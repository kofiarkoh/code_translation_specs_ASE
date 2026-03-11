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
int **flow, *visit, *delta, *prev, *stack;
void Insert(int, int);
int Maxflow(int, int, int);
int main(void)
{
  int i, x, y, v, e, s, t, maxflow = 0;
  scanf("%d %d %d", &x, &y, &e);
  v = x + y + 2;
  list = (struct node**)malloc(sizeof(struct node*) * v);
  flow = (int**)malloc(sizeof(int*) * v);
  visit = (int*)malloc(sizeof(int) * v);
  delta = (int*)malloc(sizeof(int) * v);
  prev = (int*)malloc(sizeof(int) * v);
  stack = (int*)malloc(sizeof(int) * v);
  for (i = 0; i < v; i++) {
    list[i] = NULL;
    flow[i] = (int*)calloc(v, sizeof(int));
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d", &s, &t);
    Insert(0, s + 1);
    Insert(x + t + 1, v - 1);
    Insert(s + 1, x + t + 1);
  }
  while (Maxflow(0, v - 1, v)) {
    int n = v - 1;
    do {
      flow[prev[n]][n] += delta[v-1];
      flow[n][prev[n]] -= delta[v-1];
      n = prev[n];
    } while(n);
    maxflow += delta[v-1];
  }
  printf("%d\n", maxflow);
  for (i = 0; i < v; i++) {
    free(list[i]);
    free(flow[i]);
  }
  free(list);
  free(flow);
  free(visit);
  free(delta);
  free(prev);
  free(stack);
}
void Insert(int a, int b)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->cap = 1;
  p->next = list[a];
  list[a] = p;
  p = (struct node*)malloc(sizeof(struct node));
  p->id = a;
  p->cap = 0;
  p->next = list[b];
  list[b] = p;
}
int Maxflow(int s, int t, int size)
{
  struct node *n;
  int i, reached = 0, tail = 0;
  for (i = 0; i < size; i++) {
    visit[i] = 0;
    delta[i] = INT_MAX;
    prev[i] = -1;
  }
  visit[s] = 1;
  stack[tail++] = s;
  do {
    i = stack[--tail];
    for (n = list[i]; n != NULL; n = n->next) {
      int v = n->id;
      if (!visit[v] && flow[i][v] < n->cap) {
        stack[tail++] = v;
        visit[v] = 1;
        prev[v] = i;
        delta[v] = MIN(delta[i], n->cap - flow[i][v]);
        if (v == t) reached = 1;
      }
    }
  } while (tail && !reached);
  return reached;
}
