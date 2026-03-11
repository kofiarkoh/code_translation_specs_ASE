#include<stdio.h>
#include<stdlib.h>
struct node {
  int id;
  struct node* next;
};

struct node **list;
int *seq, *group, *stack, counter, phase, except,sp;

void Insert(int, int);
int strongcomponent(int);
int main(void)
{
  int i, v, e,s ,t, q;
  scanf("%d %d", &v, &e);
  list = (struct node**)malloc(sizeof(struct node*) * v);
  seq = (int*)calloc(v, sizeof(int));
  group = (int*)malloc(sizeof(int) * v);
  stack = (int*)malloc(sizeof(int) * v);
  except = v + 1;
  for (i = 0; i < v; i++) {
    list[i] = NULL;
    group[i] = -1;
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d", &s, &t);
    Insert(s,t);
  }
  for (i = 0; i < v; i++) {
    if (!seq[i]) {
      strongcomponent(i);
    }
  }
  scanf("%d", &q);
  for (i = 0; i < q; i++) {
    scanf("%d %d", &s, &t);
    if (group[s] == group[t]) printf("1\n");
    else printf("0\n");
    free(list[i]);
  }
  free(seq);
  free(group);
  free(stack);
}
void Insert(int a, int b)
{
  struct node *p = (struct node*)malloc(sizeof(struct node));
  p->id = b;
  p->next = list[a];
  list[a] = p;
}
int strongcomponent(int point)
{
  int min;
  struct node *n;
  counter++;
  min = seq[point] = counter;
  stack[sp++] = point;
  for (n = list[point]; n != NULL; n = n->next) {
    int m;
    if (!seq[n->id]) m = strongcomponent(n->id);
    else m = seq[n->id];
    if (m < min) min = m;
  }
  if (min == seq[point]) {
    int x;
    do {
      sp--;
      x = stack[sp];
      group[x] = phase;
      seq[x] = except;
    } while (x != point);
    phase++;
  }
  return min;
}
