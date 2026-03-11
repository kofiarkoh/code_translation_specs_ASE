#include<stdio.h>
#include<stdlib.h>

struct edge {
  int a;
  int b;
  int weight;
};
int *ufp, *ufr;
struct edge *ed;

void qsort1(int, int);
int UF_find(int);
void UF_union(int, int);
int Kruskal(int, int);

int main(void)
{
  int i, j, v, e, s, t, w, cost;
  scanf("%d %d", &v, &e);
  ufp = (int*)malloc(sizeof(int) * v);
  ufr = (int*)malloc(sizeof(int) * v);
  ed = (struct edge*)malloc(sizeof(struct edge) * e);
  for (i = 0; i < v; i++) {
    ufp[i] = i;
    ufr[i] = 0;
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d %d", &s, &t, &w);
    ed[i].weight = w;
    ed[i].a = s;
    ed[i].b = t;
  }
  cost = Kruskal(v, e);
  printf("%d\n", cost);
  free(ufp);
  free(ufr);
  free(ed);
}

void qsort1(int left, int right)
{
  int pivot = ed[(left + right) / 2].weight, i = left, j = right;
  if (i >= j) return;
  do {
    while (ed[i].weight < pivot) i++;
    while (ed[j].weight > pivot) j--;
    if (i <= j) {
      struct edge temp = ed[i];
      ed[i] = ed[j];
      ed[j] = temp;
      i++;
      j--;
    }
  } while (i <= j);
  qsort1(left, j);
  qsort1(i, right);
}

int UF_find(int a)
{
  if (ufp[a] == a) return a;
  else {
    ufp[a] = UF_find(ufp[a]);
    return ufp[a];
  }
}

void UF_union(int a, int b)
{
  if (ufr[a] < ufr[b]) ufp[a] = b;
  else {
    ufp[b] = a;
    if (ufr[a] == ufr[b]) ufr[a]++;
  }
}

int Kruskal(int size, int esize)
{
  int i, selected = 0, cost = 0;
  qsort1(0, esize - 1);
  for (i = 0; i < esize; i++) {
    int ea = UF_find(ed[i].a), eb = UF_find(ed[i].b);
    if (ea != eb) {
      UF_union(ea, eb);
      cost += ed[i].weight;
      selected++;
    }
    if (selected == size - 1) break;
  }
  return cost;
}
