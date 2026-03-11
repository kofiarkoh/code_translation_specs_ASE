#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>

struct vertex {
  int dis;
  int k;
  int visited;
  int *vlist;
};

struct vertex *adj;

int *queue, n, in, out;

int main(int argc, char *argv[])
{
  int i, j, id, d, v;

  scanf("%d", &n);
  adj = (struct vertex *)malloc(sizeof(struct vertex) * (n + 1));
  queue = (int *)malloc(sizeof(int) * (n + 1));
  in = 0;
  out = 0;

  for (i = 0; i < n; i++) {
    scanf("%d", &id);
    scanf("%d", &(adj[id].k));
    adj[id].vlist = (int *)malloc(sizeof(int) * adj[id].k);
    adj[id].visited = 0;
    adj[id].dis = -1;
    for (j = 0; j < adj[id].k; j++) scanf("%d", &(adj[id].vlist[j]));
  }
  
  adj[1].dis = 0;
  adj[1].visited = 1;
  queue[in] = 1;
  in = (in + 1) % n;
  while (in != out) {
    v = queue[out];
    out = (out + 1) % n;
    for (i = 0; i < adj[v].k; i++) {
      if (adj[adj[v].vlist[i]].visited == 0) {
	adj[adj[v].vlist[i]].visited = 1;
	adj[adj[v].vlist[i]].dis = adj[v].dis + 1;
	queue[in] = adj[v].vlist[i];
	in = (in + 1) % n;
      }
    }
  }

  for (i = 1; i <= n; i++) printf("%d %d\n", i, adj[i].dis);

  return 0;
}