#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <memory.h>
#include <math.h>

#define MAX(a, b) (((a) > (b)) ? (a) : (b))
#define MIN(a, b) (((a) > (b)) ? (b) : (a))
#define SWAP(t, a, b) do{t c = a; a = b; b = c;}while(0)

typedef struct
{
  int64_t a;
  int64_t b;
} DATA;

void stable_sort(void *ptr, size_t count, size_t size,
  int (*compare)(const void *, const void *))
{
  int i = 0;
  int j = 0;
  void *temp = malloc(size);

  for(i = 0; i < count; i++)
  {
    for(j = count - 1; j > i; j--)
    {
      void *a = (void *)((uint64_t)ptr + j * size);
      void *b = (void *)((uint64_t)ptr + (j - 1) * size);

      if(compare(a, b) < 0)
      {
        memcpy(temp, a, size);
        memcpy(a, b, size);
        memcpy(b, temp, size);
      }
    }
  }

  free(temp);
}

int compare1(const void *a, const void *b)
{
  return ((DATA *)b)->a - ((DATA *)a)->a;
}

int compare2(const void *a, const void *b)
{
  return ((DATA *)b)->b - ((DATA *)a)->b;
}

int main (int argc, char **argv)
{
  int64_t n = 0;
  int64_t m = 0;
  DATA *d = NULL;
  int64_t ans = 0;
  int64_t i = 0;
  

  scanf("%lld %lld", &n, &m);

  d = (DATA *)malloc(n * sizeof(DATA));

  for(i = 0; i < n; i++)
  {
    scanf("%lld %lld", &d[i].a, &d[i].b);
  }

  qsort(d, n, sizeof(DATA), compare1);
  stable_sort(d, n, sizeof(DATA), compare2);

  for(i = 0; i < n; i++)
  {
    if(d[i].a <= m)
    {
      ans += d[i].b;
      m--;
    }
  }

  printf("%lld\n", ans);

  return 0;
}
