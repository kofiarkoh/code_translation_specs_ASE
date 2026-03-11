#include<stdio.h>
#include<stdlib.h>
#define NMAX 500
#define MMAX 50000
struct edge {
  int a;
  int b;
  int weight;
};
int ufp[NMAX], ufr[NMAX], etsize;
struct edge ed[MMAX], et[MMAX];

int compare(const void*, const void*);
void UF_init(int);
int UF_find(int);
void UF_union(int, int);
int Kruskal(int, int);

int main(void)
{
  int i, j, v, e, s, t, w, cost, tcost, eans = 0, cans = 0;
  scanf("%d %d", &v, &e);
  UF_init(v);
  for (i = 0; i < e; i++) {
    scanf("%d %d %d", &s, &t, &w);
    ed[i].weight = w;
    ed[i].a = s - 1;
    ed[i].b = t - 1;
  }
  qsort(ed, e, sizeof(struct edge), compare);
  cost = Kruskal(v, e);
  for (i = 0; i < etsize; i++) {
    UF_init(v);
    tcost = 0;
    for (j = 0; j < e; j++) {
      int jea = UF_find(ed[j].a), jeb = UF_find(ed[j].b);
      if (jea != jeb && !(et[i].a == ed[j].a && et[i].b == ed[j].b)) {
        UF_union(jea, jeb);
        tcost += ed[j].weight;
      }
    }
    if (cost != tcost) {
      eans++;
      cans += et[i].weight;
    }
  }
  printf("%d %d\n", eans, cans);
}

int compare(const void *a, const void *b)
{
  int wa = ((struct edge*)a)->weight, wb = ((struct edge*)b)->weight;
  return wa - wb;
}

void UF_init(int v)
{
  int i;
  for (i = 0; i < v; i++) {
    ufp[i] = i;
    ufr[i] = 0;
  }
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
  for (i = 0; i < esize; i++) {
    int ea = UF_find(ed[i].a), eb = UF_find(ed[i].b);
    if (ea != eb) {
      UF_union(ea, eb);
      cost += ed[i].weight;
      et[etsize++] = ed[i];
      selected++;
    }
    if (selected == size - 1) break;
  }
  return cost;
}
