#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int acs(const void *a, const void *b) { return *(int *)a - *(int *)b; }                            /* 1,2,3,4.. */
int des(const void *a, const void *b) { return *(int *)b - *(int *)a; }                            /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b) { return *(char *)a - *(char *)b; }                     /* a,b,c,d.. */
int cmp_str(const void *a, const void *b) { return strcmp(*(const char **)a, *(const char **)b); } /* aaa,aab.. */
#define min(a, b) (a < b ? a : b)
#define max(a, b) (a > b ? a : b)

int main(void)
{
  int N, K;
  int value[51], have[51] = {0}, size = 0;

  scanf("%d %d", &N, &K);

  int i, ki, ni;
  for (ni = 0; ni < N; ni++)
    scanf("%d", &value[ni]);

  int head, rear;
  int max = 0, sum;
  int tresures = min(N, K);
  for (head = 0; head <= N; head++)
  {
    for (rear = N - 1; rear >= -1; rear--)
    {
      if (head + (N - 1 - rear) > N)
        continue;
      if (head + (N - 1 - rear) > K)
        continue;
      sum = 0;
      size = 0;
      for (i = 0; i < head; i++)
      {
        sum += value[i];
        have[size++] = value[i];
      }
      for (i = N - 1; i > rear; i--)
      {
        sum += value[i];
        have[size++] = value[i];
      }
      qsort(have, size, sizeof(int), acs);
      i = K - head - (N - 1 - rear);
      ki = 0;
      while (i > 0 && have[ki] < 0 && ki < size)
      {
        sum -= have[ki++];
        i--;
      }
      max = max(max, sum);
    }
  }
  printf("%d\n", max);
  return 0;
}
