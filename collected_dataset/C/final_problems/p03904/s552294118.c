#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define NMAX 100000

int len, part, rank[NMAX+1];
int compare_sa(const void *a, const void *b);
void construct_sa(char *string, int n, int *sa);
int main(void)
{
  int i, n, k, l, lo, hi, sa[NMAX+1];
  char s[NMAX+1];
  scanf("%d%*c", &k);
  scanf("%s%*c", s);
  n = strlen(s);
  l = (n + k) / (k + 1);
  lo = 1;
  hi = n;
  construct_sa(s, n, sa);
  if (n % (k + 1) == 0) {
    int res = 0;
    for (i = 0; i < k; i++) {
      if (rank[n/(k+1)*(i+1)] > rank[res]) res = n / (k + 1) * (i + 1);
    }
    s[res+n/(k+1)] = '\0';
    printf("%s\n", s + res);
  } else {
    while (hi >= lo) {
      int mid = (lo + hi) / 2, pos = 0, res = 0;
      while (pos < n) {
        pos += rank[pos] <= mid ? l : l - 1;
        res++;
      }
      if (res <= k + 1) hi = mid - 1;
      else lo = mid + 1;
    }
    s[sa[lo]+l] = '\0';
    printf("%s\n", s + sa[lo]);
  }
}

int compare_sa(const void *a, const void *b)
{
  int i = *(int*)a, j = *(int*)b;
  if (rank[i] != rank[j]) return rank[i] - rank[j];
  else {
    int ri = (i + part <= len ? rank[i+part] : -1), rj = (j + part <= len ? rank[j+part] : -1);
    return ri - rj;
  }
}

void construct_sa(char *string, int n, int *sa)
{
  int i, tmp[NMAX+1];
  len = n;
  for (i = 0; i <= len; i++) {
    sa[i] = i;
    rank[i] = i < n ? string[i] : -1;
  }
  for (part = 1; part <= len / 2; part *= 2) {
    qsort(sa, len + 1, sizeof(int), compare_sa);
    tmp[sa[0]] = 0;
    for (i = 1; i <= len; i++) {
      tmp[sa[i]] = tmp[sa[i-1]] + (compare_sa(&sa[i-1], &sa[i]) ? 1 : 0);
    }
    for (i = 0; i <= len; i++) rank[i] = tmp[i];
  }
}
