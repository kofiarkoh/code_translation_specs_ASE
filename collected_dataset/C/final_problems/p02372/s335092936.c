#include<stdio.h>
#include<stdlib.h>

struct node {
  int id;
  int weight;
  struct node *next;
};
struct node **list;
int *maxdis, *premaxdis;

void Insert(int, int, int);
void Dfs(int, int);
void RevDfs(int, int);
int main(void)
{
  int i, n, s, t, w;
  scanf("%d", &n);
  maxdis = (int*)calloc(n, sizeof(int));
  premaxdis = (int*)calloc(n, sizeof(int));
  list = (struct node**)malloc(sizeof(struct node*) * n);
  for (i = 0; i < n; i++) list[i]= NULL;
  for (i = 0; i < n - 1; i++) {
    scanf("%d %d %d", &s, &t, &w);
    Insert(s, t, w);
  }
  Dfs(s, -1);
  RevDfs(s, -1);
  for (i = 0; i < n ;i++) {
    printf("%d\n", maxdis[i]);
    free(list[i]);
  }
  free(list);
  free(maxdis);
  free(premaxdis);
}
void Insert(int a, int b, int w)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->weight = w;
  p->next = list[a];
  list[a] = p;
  p = (struct node*)malloc(sizeof(struct node));
  p->id = a;
  p->weight = w;
  p->next = list[b];
  list[b] = p;
}
void Dfs(int root, int fa)
{
  struct node *i;
  for (i = list[root]; i != NULL; i = i->next) {
    if (i->id != fa) {
      Dfs(i->id, root);
      if (maxdis[i->id] + i->weight > maxdis[root]) {
        premaxdis[root] = maxdis[root];
        maxdis[root] = maxdis[i->id] + i->weight;
      } else if (maxdis[i->id] + i->weight > premaxdis[root]) {
        premaxdis[root] = maxdis[i->id] + i->weight;
      }
    }
  }
}
void RevDfs(int leaf, int fa)
{
  struct node *i;
  for (i = list[leaf]; i != NULL; i = i->next) {
    if (i->id != fa) {
      if (maxdis[i->id] + i->weight == maxdis[leaf]) {
        if (premaxdis[leaf] + i->weight > maxdis[i->id]) {
          premaxdis[i->id] = maxdis[i->id];
          maxdis[i->id] = premaxdis[leaf] + i->weight;
        } else if (premaxdis[leaf] + i->weight > premaxdis[i->id]) {
          premaxdis[i->id] = premaxdis[leaf] + i->weight;
        }
      }else {
        if (maxdis[leaf] + i->weight > maxdis[i->id]) {
          premaxdis[i->id] = maxdis[i->id];
          maxdis[i->id] = maxdis[leaf] + i->weight;
        } else if (maxdis[leaf] + i->weight > premaxdis[i->id]) {
          premaxdis[i->id] = maxdis[leaf] + i->weight;
        }
      }
      RevDfs(i->id, leaf);
    }
  }
}
