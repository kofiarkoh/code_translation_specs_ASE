#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define abs(x) ((x) > 0 ? (x) : -(x))
#define MOD (ll)1000000007 //10^9 + 7
#define endl printf("\n")
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (n); i++)
#define rep2(i, n, m) for (int i = (n), i < (m); i++)

ll d[400][400];

void wf(int n) {
  for (int k = 0; k < n; k++) {
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++)  {
	d[i][j] = Min(d[i][j], d[i][k] + d[k][j]);
      }
    }
  }
}

int
main(int argc, char *argv[])
{
  
  int n;
  int s[400][400];
  scanf("%d", &n);

  rep(i, n)rep(j, n) {
    d[i][j] = 10000000000;
    if(i == j) d[i][j] = 0;
  }
  
  rep(i, n) rep(j, n) {
    scanf("%1d", &s[i][j]);
    if (s[i][j] == 1) d[i][j] = 1;
  }

  wf(n);


  ll max = 0;
  rep(i, n) rep(j, n) {
    max = Max(max, d[i][j]);
    rep(k, n) {
      if (i != j && s[i][k] == 1 && s[j][k] == 1 && s[i][j] == 1) {
	printf("-1\n");
	return 0;
      }
    }
  }
  printf("%lld\n", max + (ll)1);

  return 0;
}
	  
