#include <stdio.h>
#include <stdlib.h>
#define NMAX 100000
struct node {
  int id;
  struct node *next;
};
struct node *list[NMAX];
void Insert(int a, int b);
int Dfs(int v, int c, int *color);
int main(void)
{
  int i, n, m, a, b, color[NMAX] = {0};
  long long int wb[2] = {0};
  scanf("%d %d", &n, &m);
  for (i = 0; i < n; i++) list[i] = NULL;
  for (i = 0; i < m; i++) {
    scanf("%d %d", &a, &b);
    Insert(a - 1, b - 1);
  }
  for (i = 0; i < n; i++) {
    if (color[i] == 0) {
      if (!Dfs(i, 1, color)) {
        printf("%lld\n", (long long int)(n - 1) * n / 2 - m);
        return 0;
      }
    }
  }
  for (i = 0; i < n; i++) {
     wb[(color[i]+1)/2]++;
     free(list[i]);
  }
  printf("%lld\n", wb[0] * wb[1] - m);
}

void Insert(int a, int b)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->next = list[a];
  list[a] = p;
  p = (struct node*)malloc(sizeof(struct node));
  p->id = a;
  p->next = list[b];
  list[b] = p;
}

int Dfs(int v, int c, int *color)
{
  struct node *n;
  color[v] = c;
  for (n = list[v]; n != NULL; n = n->next) {
    if (color[n->id] == c) return 0;
    else if (color[n->id] == 0 && !Dfs(n->id, -c, color)) return 0;
  }
  return 1;
}