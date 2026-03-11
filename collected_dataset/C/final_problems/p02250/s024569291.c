#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_T 1000000
#define MAX_P 1000

int kmpMatcher(char *t, int tlen, char *p, int plen);
void computePrefixFunction(char *p, int plen, int *pi);
int main();

/* t, p is start from 1. */
int kmpMatcher(char *t, int tlen, char *p, int plen) {
  int n, m;
  int pi_start_from_0[MAX_P];
  int *pi = pi_start_from_0 - 1;
  int q, i;

  n = tlen;
  m = plen;
  computePrefixFunction(p, plen, pi);
  q = 0;
  for (i = 1; i <= n; i++) {
    while (q > 0 && p[q+1] != t[i])
      q = pi[q];
    if (p[q+1] == t[i])
      q++;
    if (q == m) {
      /* printf("pattern is found at shift %d.\n", i - m); */
      return 1;
      q = pi[q];
    }
  }
  return 0;
}

/* p, pi is start from 1. */
void computePrefixFunction(char *p, int plen, int *pi) {
  int m, k, q;
  m = plen;
  pi[1] = 0;
  k = 0;
  for (q = 2; q <= m; q++) {
    while (k > 0 && p[k+1] != p[q])
      k = pi[k];
    if (p[k+1] == p[q])
      k++;
    pi[q] = k;
  }
}

int main() {
  char *t_start_from_0 = (char *)malloc(MAX_T+1);
  char *t = t_start_from_0 - 1;
  int tlen = 0;
  char p_start_from_0[MAX_P+1];
  char *p = p_start_from_0 - 1;
  int plen = 0;
  int q = 0;
  int i;

  if (t_start_from_0 == NULL) {
    printf("memory shortage.\n");
    return 1;
  }

  scanf("%s", t_start_from_0);
  tlen = strlen(t_start_from_0);
  scanf("%d", &q);

  for (i = 0; i < q; i++) {
    scanf("%s", p_start_from_0);
    plen = strlen(p_start_from_0);
    printf("%d\n", kmpMatcher(t, tlen, p, plen));
  }

  free(t_start_from_0);
  return 0;
}