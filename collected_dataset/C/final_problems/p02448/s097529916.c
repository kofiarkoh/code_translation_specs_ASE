#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int value;
  int weight;
  char type;
  long int date;
  char name[21];
} Item;

int compar(const void *a, const void *b)
{
  Item *pa = (Item *)a;
  Item *pb = (Item *)b;
  if (pa->value > pb->value)
    return 1;
  if (pa->value == pb->value) {
    if (pa->weight > pb->weight) return 1;
    if (pa->weight == pb->weight) {
      if (pa->type > pb->type) return 1;
      if (pa->type == pb->type) {
        if (pa->date > pb->date) return 1;
        if (pa->date == pb->date) {
          int c=strcmp(pa->name, pb->name);
          if (c>0) return 1;
          else if (c==0) return 0;
        }
      }
    }
  }
  return -1;
}

int main()
{
  int i, j, n, t, len=1;
  fscanf(stdin, "%d", &n);
  Item items[n];
  for (i=0; i<n; i++)
    fscanf(stdin, "%d %d %c %ld %s\n", &items[i].value, &items[i].weight, &items[i].type, &items[i].date, items[i].name);
  
  qsort(items, n, sizeof(Item), compar);
  
  for (i=0; i<n; i++)
    fprintf(stdout, "%d %d %c %ld %s\n", items[i].value, items[i].weight, items[i].type, items[i].date, items[i].name);

  return 0;
}

