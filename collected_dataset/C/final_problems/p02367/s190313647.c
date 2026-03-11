#include<stdio.h>
#include<stdlib.h>

struct node {
  int id;
  struct node *next;
};
struct node **list;
int *seq, **art, counter;

void Insert(int, int);
int Biconnected(int, int);
int main(void)
{
  int i, j, v, e, s, t;
  scanf("%d %d", &v, &e);
  list = (struct node**)malloc(sizeof(struct node*) * v);
  art = (int**)malloc(sizeof(int*) * v);
  seq = (int*)calloc(v, sizeof(int));
  for (i = 0; i < v; i++) {
    list[i] = NULL;
    art[i] = (int*)calloc(v, sizeof(int));
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d", &s, &t);
    Insert(s,t);
  }
  Biconnected(0, -1);
  for (i = 0; i < v; i++) {
    for (j = 0; j < v; j++) {
      if (art[i][j]) printf("%d %d\n", i, j);
    }
    free(art[i]);
    free(list[i]);
  }
  free(list);
  free(seq);
  free(art);
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
int Biconnected(int point, int pre)
{
  int min;
  struct node *n;
  counter++;
  seq[point] = counter;
  min = counter;
  for(n = list[point]; n != NULL; n = n->next) {
    if (!seq[n->id]) {
      int m;
      m = Biconnected(n->id, point);
      if (m < min) min = m;
      if (m > seq[point]) {
        if (point < n->id) art[point][n->id] = 1;
        else art[n->id][point] = 1;
      }
    } else if (n->id != pre) {
      if (seq[n->id] < min) {
        min = seq[n->id];
      }
    }
  }
  return min;
}
